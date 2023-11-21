// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use std::collections::HashMap;
use std::str::FromStr;

use bitcoin::bip32::{DerivationPath, ExtendedPrivKey};
use bitcoin::secp256k1::Secp256k1;
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::crypto::{decrypt_private_key, CryptoError};
use crate::error::Error;
use crate::key::OperationSigningKey;
use crate::objects::account::Account;
use crate::objects::api_token::ApiToken;
use crate::objects::compliance_provider::ComplianceProvider;
use crate::objects::currency_amount::{self, CurrencyAmount};
use crate::objects::fee_estimate::FeeEstimate;
use crate::objects::incoming_payment::IncomingPayment;
use crate::objects::invoice;
use crate::objects::invoice::Invoice;
use crate::objects::invoice_data::InvoiceData;
use crate::objects::invoice_type::InvoiceType;
use crate::objects::lightning_fee_estimate_output::LightningFeeEstimateOutput;
use crate::objects::outgoing_payment::OutgoingPayment;
use crate::objects::permission::Permission;
use crate::objects::region_code::RegionCode;
use crate::objects::risk_rating::RiskRating;
use crate::objects::uma_invitation::UmaInvitation;
use crate::objects::withdrawal_mode::WithdrawalMode;
use crate::objects::withdrawal_request::WithdrawalRequest;
use crate::objects::{account, invoice_data, uma_invitation};
use crate::objects::{api_token, incoming_payment, outgoing_payment};
use crate::objects::{bitcoin_network, withdrawal_request};
use crate::objects::{fee_estimate, lightning_fee_estimate_output};
use crate::request::auth_provider::AuthProvider;
use crate::request::requester::Requester;
use crate::types::get_entity::GetEntity;
use crate::types::graphql_requester::GraphQLRequester;

const SIGNING_KEY_PATH: &str = "m/5";

pub struct LightsparkClient<T: OperationSigningKey> {
    pub requester: Requester,
    signing_keys: HashMap<String, T>,
}

impl<K: OperationSigningKey> LightsparkClient<K> {
    pub fn new<T: AuthProvider>(auth_provider: T) -> Result<Self, Error> {
        let requester = Requester::new(auth_provider)?;
        Ok(LightsparkClient {
            requester,
            signing_keys: HashMap::new(),
        })
    }

    pub fn provide_master_seed(
        &mut self,
        node_id: &str,
        master_seed: Vec<u8>,
        network: bitcoin_network::BitcoinNetwork,
    ) -> Result<(), Error> {
        let network: bitcoin::Network = match network {
            bitcoin_network::BitcoinNetwork::Mainnet => bitcoin::Network::Bitcoin,
            bitcoin_network::BitcoinNetwork::Testnet => bitcoin::Network::Testnet,
            bitcoin_network::BitcoinNetwork::Regtest => bitcoin::Network::Regtest,
            bitcoin_network::BitcoinNetwork::Signet => bitcoin::Network::Signet,
        };
        let master_private_key = ExtendedPrivKey::new_master(network, &master_seed)
            .map_err(|e| Error::CryptoError(CryptoError::Bip32Error(e)))?;
        let secp = Secp256k1::new();
        let node_key_path = DerivationPath::from_str(SIGNING_KEY_PATH)
            .map_err(|e| Error::CryptoError(CryptoError::Bip32Error(e)))?;
        let signing_key = master_private_key
            .derive_priv(&secp, &node_key_path)
            .map_err(|e| Error::CryptoError(CryptoError::Bip32Error(e)))?;

        let key = K::new(signing_key.private_key.secret_bytes().to_vec());
        self.load_node_signing_key(node_id, key);
        Ok(())
    }

    pub async fn get_bitcoin_fee_estimates(
        &self,
        bitcoin_network: bitcoin_network::BitcoinNetwork,
    ) -> Result<FeeEstimate, Error> {
        let query = format!(
            "
            query BitcoinFeeEstimate(
                $bitcoin_network: BitcoinNetwork!
            ) {{
                bitcoin_fee_estimate(network: $bitcoin_network) {{
                    ...FeeEstimateFragment
                }}
            }}
            {}",
            fee_estimate::FRAGMENT
        );

        let mut variables = HashMap::new();
        variables.insert("bitcoin_network", bitcoin_network);

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self.requester.execute_graphql(&query, Some(value)).await?;
        let result = serde_json::from_value(json["bitcoin_fee_estimate"].clone())
            .map_err(Error::JsonError)?;
        Ok(result)
    }

    pub async fn get_lightning_fee_estimate_for_node(
        &self,
        node_id: &str,
        destination_node_public_key: &str,
        amount_msats: i64,
    ) -> Result<CurrencyAmount, Error> {
        let query = format!(
            "query LightningFeeEstimateForNode(
                $node_id: ID!
                $destination_node_public_key: String!
                $amount_msats: Long!
              ) {{
                lightning_fee_estimate_for_node(input: {{
                  node_id: $node_id,
                  destination_node_public_key: $destination_node_public_key,
                  amount_msats: $amount_msats
                }}) {{
                  ...LightningFeeEstimateOutputFragment
                }}
              }}
              {}",
            lightning_fee_estimate_output::FRAGMENT
        );

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("node_id", node_id.into());
        variables.insert(
            "destination_node_public_key",
            destination_node_public_key.into(),
        );
        variables.insert("amount_msats", amount_msats.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self.requester.execute_graphql(&query, Some(value)).await?;
        let result: LightningFeeEstimateOutput =
            serde_json::from_value(json["lightning_fee_estimate_for_node"].clone())
                .map_err(Error::JsonError)?;
        Ok(result.fee_estimate)
    }

    pub async fn get_lightning_fee_estimate_for_invoice(
        &self,
        node_id: &str,
        encoded_payment_request: &str,
        amount_msats: i64,
    ) -> Result<CurrencyAmount, Error> {
        let query = format!(
            "query LightningFeeEstimateForInvoice(
                $node_id: ID!
                $encoded_payment_request: String!
                $amount_msats: Long!
              ) {{
                lightning_fee_estimate_for_invoice(input: {{
                  node_id: $node_id,
                  encoded_payment_request: $encoded_payment_request,
                  amount_msats: $amount_msats
                }}) {{
                  ...LightningFeeEstimateOutputFragment
                }}
              }}
              {}",
            lightning_fee_estimate_output::FRAGMENT
        );

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("node_id", node_id.into());
        variables.insert("encoded_payment_request", encoded_payment_request.into());
        variables.insert("amount_msats", amount_msats.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self.requester.execute_graphql(&query, Some(value)).await?;
        let result: LightningFeeEstimateOutput =
            serde_json::from_value(json["lightning_fee_estimate_for_node"].clone())
                .map_err(Error::JsonError)?;
        Ok(result.fee_estimate)
    }

    pub async fn get_current_account(&self) -> Result<Account, Error> {
        let query = format!(
            "
            query GetCurrentAccount {{
                current_account {{
                    ...AccountFragment
                }}
            }}            
            {}
            ",
            account::FRAGMENT
        );

        let json = self.requester.execute_graphql(&query, None).await?;
        let result =
            serde_json::from_value(json["current_account"].clone()).map_err(Error::JsonError)?;
        Ok(result)
    }

    pub async fn create_api_token(
        &self,
        name: &str,
        transact: bool,
        test_mode: bool,
    ) -> Result<(ApiToken, String), Error> {
        let operation = format!(
            "
            mutation CreateApiToken(
                $name: String!
                $permissions: [Permission!]!
            ) {{
                create_api_token(input: {{
                    name: $name
                    permissions: $permissions
                }}) {{
                    api_token {{
                        ...ApiTokenFragment
                    }}
                    client_secret
                }}
            }}
            {}
            ",
            api_token::FRAGMENT
        );

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("name", name.into());
        let permission = match (test_mode, transact) {
            (true, true) => vec![Permission::RegtestView, Permission::RegtestTransact],
            (true, false) => vec![Permission::RegtestView],
            (false, true) => vec![Permission::MainnetView, Permission::MainnetTransact],
            (false, false) => vec![Permission::MainnetView],
        };
        variables.insert("permissions", permission.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self
            .requester
            .execute_graphql(&operation, Some(value))
            .await?;

        let result = serde_json::from_value(json["create_api_token"]["api_token"].clone())
            .map_err(Error::JsonError)?;
        let client_secret = json["create_api_token"]["client_secret"]
            .clone()
            .as_str()
            .unwrap_or("")
            .to_owned();
        Ok((result, client_secret))
    }

    pub async fn delete_api_token(&self, api_token_id: &str) -> Result<(), Error> {
        let operation = "
        mutation DeleteApiToken(
            $api_token_id: ID!
        ) {
            delete_api_token(input: {
                api_token_id: $api_token_id
            }) {
                __typename
            }
        }
        ";

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("api_token_id", api_token_id.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        self.requester
            .execute_graphql(operation, Some(value))
            .await?;
        Ok(())
    }

    pub async fn get_entity<T>(&self, id: &str) -> Result<T, Error>
    where
        T: GetEntity,
    {
        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("id", id.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self
            .requester
            .execute_graphql(T::get_entity_query().as_str(), Some(value))
            .await?;

        let result = serde_json::from_value(json["entity"].clone()).map_err(Error::JsonError)?;
        Ok(result)
    }

    pub async fn create_invoice(
        &self,
        node_id: &str,
        amount_msats: i64,
        memo: Option<&str>,
        invoice_type: Option<InvoiceType>,
    ) -> Result<Invoice, Error> {
        let operation = format!(
            "mutation CreateInvoice(
                $node_id: ID!
                $amount_msats: Long!
                $memo: String
                $invoice_type: InvoiceType
            ) {{
                create_invoice(input: {{
                    node_id: $node_id
                    amount_msats: $amount_msats
                    memo: $memo
                    invoice_type: $invoice_type
                }}) {{
                    invoice {{
                        ...InvoiceFragment
                    }}
                }}
            }}
            
            {}
            ",
            invoice::FRAGMENT
        );

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("node_id", node_id.into());
        variables.insert("memo", memo.into());
        variables.insert("amount_msats", amount_msats.into());
        variables.insert("invoice_type", invoice_type.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self
            .requester
            .execute_graphql(&operation, Some(value))
            .await?;

        let result = serde_json::from_value(json["create_invoice"]["invoice"].clone())
            .map_err(Error::JsonError)?;
        Ok(result)
    }

    pub async fn create_lnurl_invoice(
        &self,
        node_id: &str,
        amount_msats: i64,
        metadata: &str,
    ) -> Result<Invoice, Error> {
        let operation = format!(
            "mutation CreateLnurlInvoice(
                $node_id: ID!
                $amount_msats: Long!
                $metadata_hash: String!
            ) {{
                create_lnurl_invoice(input: {{
                    node_id: $node_id
                    amount_msats: $amount_msats
                    metadata_hash: $metadata_hash
                }}) {{
                    invoice {{
                        ...InvoiceFragment
                    }}
                }}
            }}
            
            {}
            ",
            invoice::FRAGMENT
        );

        let mut hasher = Sha256::new();
        hasher.update(metadata.as_bytes());

        let metadata_hash = hex::encode(hasher.finalize());

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("node_id", node_id.into());
        variables.insert("amount_msats", amount_msats.into());
        variables.insert("metadata_hash", metadata_hash.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self
            .requester
            .execute_graphql(&operation, Some(value))
            .await?;

        let result = serde_json::from_value(json["create_lnurl_invoice"]["invoice"].clone())
            .map_err(Error::JsonError)?;
        Ok(result)
    }

    pub async fn fund_node(
        &self,
        node_id: &str,
        amount_sats: i64,
    ) -> Result<CurrencyAmount, Error> {
        let operation = format!(
            "mutation FundNode(
                $node_id: ID!,
                $amount_sats: Long
            ) {{
                fund_node(input: {{ node_id: $node_id, amount_sats: $amount_sats }}) {{
                    amount {{
                        ...CurrencyAmountFragment
                    }}
                }}
            }}            
            
            {}
            ",
            currency_amount::FRAGMENT
        );

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("node_id", node_id.into());
        variables.insert("amount_sats", amount_sats.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self
            .requester
            .execute_graphql(&operation, Some(value))
            .await?;

        let result = serde_json::from_value(json["fund_node"]["amount"].clone())
            .map_err(Error::JsonError)?;
        Ok(result)
    }

    fn load_node_signing_key(&mut self, node_id: &str, signing_key: K) {
        self.signing_keys.insert(node_id.to_owned(), signing_key);
    }

    fn get_node_signing_key(&self, node_id: &str) -> Result<K, Error> {
        return self
            .signing_keys
            .get(node_id)
            .cloned()
            .ok_or(Error::SigningKeyNotFound);
    }

    pub async fn get_decoded_payment_request(
        &self,
        encoded_payment_request: &str,
    ) -> Result<InvoiceData, Error> {
        let operation = format!(
            "
            query DecodedPaymentRequest(
                $encoded_payment_request: String!
            ) {{
                decoded_payment_request(encoded_payment_request: $encoded_payment_request) {{
                    __typename
                    ... on InvoiceData {{
                        ...InvoiceDataFragment
                    }}
                }}
            }}

            {}            
            ",
            invoice_data::FRAGMENT
        );

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("encoded_payment_request", encoded_payment_request.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self
            .requester
            .execute_graphql(&operation, Some(value))
            .await?;

        let result = serde_json::from_value(json["decoded_payment_request"].clone())
            .map_err(Error::JsonError)?;
        Ok(result)
    }

    pub async fn recover_node_signing_key(
        &mut self,
        node_id: &str,
        node_password: &str,
    ) -> Result<Vec<u8>, Error> {
        let operation = "query RecoverNodeSigningKey(
                $node_id: ID!
            ) {
                entity(id: $node_id) {
                    ... on LightsparkNodeWithOSK {
                        encrypted_signing_private_key {
                            encrypted_value
                            cipher
                        }
                    }
                }
            }"
        .to_string();

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("node_id", node_id.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self
            .requester
            .execute_graphql(&operation, Some(value))
            .await?;

        let encrypted_key = json["entity"]["encrypted_signing_private_key"]["encrypted_value"]
            .as_str()
            .ok_or(Error::GraphqlError("missing encrypted_value".to_owned()))?;
        let cipher = json["entity"]["encrypted_signing_private_key"]["cipher"]
            .as_str()
            .ok_or(Error::GraphqlError("missing cipher".to_owned()))?;

        let decrypted_private_key = decrypt_private_key(cipher, encrypted_key, node_password)
            .map_err(Error::CryptoError)?;
        let key = K::new(decrypted_private_key.clone());
        self.load_node_signing_key(node_id, key);
        Ok(decrypted_private_key)
    }

    pub async fn pay_invoice(
        &self,
        node_id: &str,
        encoded_invoice: &str,
        timeout_secs: i32,
        amount_msats: Option<i64>,
        maximum_fees_msats: i64,
    ) -> Result<OutgoingPayment, Error> {
        let operation = format!(
            "
        mutation PayInvoice(
            $node_id: ID!
            $encoded_invoice: String!
            $timeout_secs: Int!
            $maximum_fees_msats: Long!
            $amount_msats: Long
        ) {{
            pay_invoice(input: {{
                node_id: $node_id
                encoded_invoice: $encoded_invoice
                timeout_secs: $timeout_secs
                maximum_fees_msats: $maximum_fees_msats
                amount_msats: $amount_msats
            }}) {{
                payment {{
                    ...OutgoingPaymentFragment
                }}
            }}
        }}
        
        {}
        ",
            outgoing_payment::FRAGMENT
        );

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("node_id", node_id.into());
        variables.insert("encoded_invoice", encoded_invoice.into());
        if let Some(amount_msats) = amount_msats {
            variables.insert("amount_msats", amount_msats.into());
        }
        variables.insert("timeout_secs", timeout_secs.into());
        variables.insert("maximum_fees_msats", maximum_fees_msats.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;

        let signing_key = self.get_node_signing_key(node_id)?;
        let json = self
            .requester
            .execute_graphql_signing(&operation, Some(value), Some(signing_key))
            .await?;

        let result = serde_json::from_value(json["pay_invoice"]["payment"].clone())
            .map_err(Error::JsonError)?;
        Ok(result)
    }

    pub async fn send_payment(
        &self,
        node_id: &str,
        destination_public_key: &str,
        timeout_secs: i32,
        amount_msats: i64,
        maximum_fees_msats: i64,
    ) -> Result<OutgoingPayment, Error> {
        let operation = format!(
            "
        mutation SendPayment(
            $node_id: ID!
            $destination_public_key: String!
            $amount_msats: Long!
            $timeout_secs: Int!
            $maximum_fees_msats: Long!
        ) {{
            send_payment(input: {{
                node_id: $node_id
                destination_public_key: $destination_public_key
                amount_msats: $amount_msats
                timeout_secs: $timeout_secs
                maximum_fees_msats: $maximum_fees_msats
            }}) {{
                payment {{
                    ...OutgoingPaymentFragment
                }}
            }}
        }}            
        {}
        ",
            outgoing_payment::FRAGMENT
        );

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("node_id", node_id.into());
        variables.insert("destination_public_key", destination_public_key.into());
        variables.insert("amount_msats", amount_msats.into());
        variables.insert("timeout_secs", timeout_secs.into());
        variables.insert("maximum_fees_msats", maximum_fees_msats.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;

        let signing_key = self.get_node_signing_key(node_id)?;

        let json = self
            .requester
            .execute_graphql_signing(&operation, Some(value), Some(signing_key))
            .await?;

        let result = serde_json::from_value(json["send_payment"]["payment"].clone())
            .map_err(Error::JsonError)?;
        Ok(result)
    }

    pub async fn execute_graphql_request(
        &self,
        operation: &str,
        variables: HashMap<&str, Value>,
    ) -> Result<Value, Error> {
        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self
            .requester
            .execute_graphql(operation, Some(value))
            .await?;
        Ok(json)
    }

    pub async fn execute_graphql_request_variable(
        &self,
        operation: &str,
        variables: Value,
    ) -> Result<Value, Error> {
        let json = self
            .requester
            .execute_graphql(operation, Some(variables))
            .await?;
        Ok(json)
    }

    pub async fn create_node_wallet_address(&self, node_id: &str) -> Result<String, Error> {
        let operation = "mutation CreateNodeWalletAddress(
                $node_id: ID!
            ) {
                create_node_wallet_address(input: {
                    node_id: $node_id
                }) {
                    wallet_address
                }
            }"
        .to_string();

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("node_id", node_id.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self
            .requester
            .execute_graphql(&operation, Some(value))
            .await?;

        if let Some(result) = json["create_node_wallet_address"]["wallet_address"].as_str() {
            Ok(result.to_owned())
        } else {
            Err(Error::GraphqlError("missing wallet_address".to_owned()))
        }
    }

    pub async fn request_withdrawal(
        &self,
        node_id: &str,
        bitcoin_address: &str,
        amount_sats: i64,
        withdrawal_mode: WithdrawalMode,
    ) -> Result<WithdrawalRequest, Error> {
        let operation = format!(
            "
        mutation RequestWithdrawal(
            $node_id: ID!
            $amount_sats: Long!
            $bitcoin_address: String!
            $withdrawal_mode: WithdrawalMode!
        ) {{
            request_withdrawal(input: {{
                node_id: $node_id
                amount_sats: $amount_sats
                bitcoin_address: $bitcoin_address
                withdrawal_mode: $withdrawal_mode
            }}) {{
                request {{
                    ...WithdrawalRequestFragment
                }}
            }}
        }}
        
        {}",
            withdrawal_request::FRAGMENT
        );

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("node_id", node_id.into());
        variables.insert("amount_sats", amount_sats.into());
        variables.insert("bitcoin_address", bitcoin_address.into());
        variables.insert("withdrawal_mode", withdrawal_mode.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let signing_key = self.get_node_signing_key(node_id)?;
        let json = self
            .requester
            .execute_graphql_signing(&operation, Some(value), Some(signing_key))
            .await?;

        let result = serde_json::from_value(json["request_withdrawal"]["request"].clone())
            .map_err(Error::JsonError)?;
        Ok(result)
    }

    pub async fn create_test_mode_invoice(
        &self,
        node_id: &str,
        amount_msats: i64,
        memo: Option<&str>,
        invoice_type: Option<InvoiceType>,
    ) -> Result<String, Error> {
        let mutation = "
            mutation CreateTestModeInvoice(
                $node_id: ID!
                $amount_msats: Long!
                $memo: String
                $invoice_type: InvoiceType
            ) {
                create_test_mode_invoice(input: {
                    local_node_id: $node_id
                    amount_msats: $amount_msats
                    memo: $memo
                    invoice_type: $invoice_type
                }) {
                    encoded_payment_request
                }
            }";

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("node_id", node_id.into());
        variables.insert("memo", memo.into());
        variables.insert("amount_msats", amount_msats.into());
        variables.insert("invoice_type", invoice_type.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;

        let json = self
            .requester
            .execute_graphql(mutation, Some(value))
            .await?;

        let result = json["create_test_mode_invoice"]["encoded_payment_request"].clone();
        Ok(result.to_string())
    }

    pub async fn create_test_mode_payment(
        &self,
        node_id: &str,
        encoded_invoice: &str,
        amount_msats: Option<i64>,
    ) -> Result<IncomingPayment, Error> {
        let mutation = format!(
            "
            mutation CreateTestModePayment(
                $node_id: ID!
                $encoded_invoice: String!
                $amount_msats: Long
            ) {{
                create_test_mode_payment(input: {{
                    local_node_id: $node_id
                    encoded_invoice: $encoded_invoice
                    amount_msats: $amount_msats
                }}) {{
                    incoming_payment {{
                        ...IncomingPaymentFragment
                    }}
                }}
            }}

            {}          
            ",
            incoming_payment::FRAGMENT
        );

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("node_id", node_id.into());
        variables.insert("encoded_invoice", encoded_invoice.into());
        if let Some(amount_msats) = amount_msats {
            variables.insert("amount_msats", amount_msats.into());
        }

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;

        let signing_key = self.get_node_signing_key(node_id)?;
        let json = self
            .requester
            .execute_graphql_signing(&mutation, Some(value), Some(signing_key))
            .await?;

        let result =
            serde_json::from_value(json["create_test_mode_payment"]["incoming_payment"].clone())
                .map_err(Error::JsonError)?;
        Ok(result)
    }

    pub async fn create_uma_invoice(
        &self,
        node_id: &str,
        amount_msats: i64,
        metadata: &str,
        expiry_secs: Option<i32>,
    ) -> Result<Invoice, Error> {
        let mutation = format!(
            "mutation CreateUmaInvoice(
            $node_id: ID!
            $amount_msats: Long!
            $metadata_hash: String!
            $expiry_secs: Int
        ) {{
            create_uma_invoice(input: {{
                node_id: $node_id
                amount_msats: $amount_msats
                metadata_hash: $metadata_hash
                expiry_secs: $expiry_secs
            }}) {{
                invoice {{
                    ...InvoiceFragment
                }}
            }}
        }}
        
        {}
        ",
            invoice::FRAGMENT
        );

        let mut hasher = Sha256::new();
        hasher.update(metadata.as_bytes());

        let metadata_hash = hex::encode(hasher.finalize());

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("node_id", node_id.into());
        variables.insert("amount_msats", amount_msats.into());
        variables.insert("metadata_hash", metadata_hash.into());
        if let Some(expiry_secs) = expiry_secs {
            variables.insert("expiry_secs", expiry_secs.into());
        }

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self
            .requester
            .execute_graphql(&mutation, Some(value))
            .await?;

        let result = serde_json::from_value(json["create_uma_invoice"]["invoice"].clone())
            .map_err(Error::JsonError)?;
        Ok(result)
    }

    pub async fn pay_uma_invoice(
        &self,
        node_id: &str,
        encoded_invoice: &str,
        timeout_secs: i32,
        maximum_fees_msats: i64,
        amount_msats: Option<i64>,
    ) -> Result<OutgoingPayment, Error> {
        let operation = format!(
            "mutation PayUmaInvoice(
            $node_id: ID!
            $encoded_invoice: String!
            $timeout_secs: Int!
            $maximum_fees_msats: Long!
            $amount_msats: Long
        ) {{
            pay_uma_invoice(input: {{
                node_id: $node_id
                encoded_invoice: $encoded_invoice
                timeout_secs: $timeout_secs
                maximum_fees_msats: $maximum_fees_msats
                amount_msats: $amount_msats
            }}) {{
                payment {{
                    ...OutgoingPaymentFragment
                }}
            }}
        }}
        
        {}
        ",
            outgoing_payment::FRAGMENT
        );

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("node_id", node_id.into());
        variables.insert("encoded_invoice", encoded_invoice.into());
        if let Some(amount_msats) = amount_msats {
            variables.insert("amount_msats", amount_msats.into());
        }
        variables.insert("timeout_secs", timeout_secs.into());
        variables.insert("maximum_fees_msats", maximum_fees_msats.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;

        let signing_key = self.get_node_signing_key(node_id)?;
        let json = self
            .requester
            .execute_graphql_signing(&operation, Some(value), Some(signing_key))
            .await?;

        let result = serde_json::from_value(json["pay_uma_invoice"]["payment"].clone())
            .map_err(Error::JsonError)?;
        Ok(result)
    }

    pub async fn screen_node(
        &self,
        provider: ComplianceProvider,
        destination_node_public_key: &str,
    ) -> Result<RiskRating, Error> {
        let operation = "mutation ScreenNode(
            $provider: ComplianceProvider!
            $node_pubkey: String!
        ) {
            screen_node(input: {
                provider: $provider
                node_pubkey: $node_pubkey
            }) {
                rating
            }
        }";

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("provider", provider.into());
        variables.insert("node_pubkey", destination_node_public_key.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self
            .requester
            .execute_graphql(operation, Some(value))
            .await?;

        let result = serde_json::from_value(json["screen_node"]["rating"].clone())
            .map_err(Error::JsonError)?;
        Ok(result)
    }

    /// Creates an UMA invitation. If you are part of the incentive program, you should use
    /// `create_uma_invitation_with_incentives`.
    pub async fn create_uma_invitation(&self, inviter_uma: &str) -> Result<UmaInvitation, Error> {
        let operation = format!(
            "mutation CreateUmaInvitation(
            $inviter_uma: String!
        ) {{
            create_uma_invitation(input: {{
                inviter_uma: $inviter_uma
            }}) {{
                invitation {{
                    ...UmaInvitationFragment
                }}
            }}
        }}

        {}
        ",
            uma_invitation::FRAGMENT
        );

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("inviter_uma", inviter_uma.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self
            .requester
            .execute_graphql(&operation, Some(value))
            .await?;

        let result = serde_json::from_value(json["create_uma_invitation"]["invitation"].clone())
            .map_err(Error::JsonError)?;
        Ok(result)
    }

    /// Creates an UMA invitation as part of the incentive program. If you are not part of the
    /// incentive program, you should use `create_uma_invitation`.
    pub async fn create_uma_invitation_with_incentives(
        &self,
        inviter_uma: &str,
        inviter_phone_number_e164: &str,
        inviter_region: RegionCode,
    ) -> Result<UmaInvitation, Error> {
        let operation = format!(
            "mutation CreateUmaInvitationWithIncentives(
            $inviter_uma: String!
            $inviter_phone_hash: String!
            $inviter_region: RegionCode!
        ) {{
            create_uma_invitation_with_incentives(input: {{
                inviter_uma: $inviter_uma
                inviter_phone_hash: $inviter_phone_hash
                inviter_region: $inviter_region
            }}) {{
                invitation {{
                    ...UmaInvitationFragment
                }}
            }}
        }}

        {}
        ",
            uma_invitation::FRAGMENT
        );

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("inviter_uma", inviter_uma.into());
        let inviter_phone_hash = self.hash_phone_number(inviter_phone_number_e164)?;
        variables.insert("inviter_phone_hash", inviter_phone_hash.into());
        variables.insert("inviter_region", inviter_region.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self
            .requester
            .execute_graphql(&operation, Some(value))
            .await?;

        let result = serde_json::from_value(
            json["create_uma_invitation_with_incentives"]["invitation"].clone(),
        )
        .map_err(Error::JsonError)?;
        Ok(result)
    }

    /// Claims an UMA invitation. If you are part of the incentive program, you should use
    /// `claim_uma_invitation_with_incentives`.
    pub async fn claim_uma_invitation(
        &self,
        invitation_code: &str,
        invitee_uma: &str,
    ) -> Result<UmaInvitation, Error> {
        let operation = format!(
            "mutation ClaimUmaInvitation(
            $invitation_code: String!
            $invitee_uma: String!
        ) {{
            claim_uma_invitation(input: {{
                invitation_code: $invitation_code
                invitee_uma: $invitee_uma
            }}) {{
                invitation {{
                    ...UmaInvitationFragment
                }}
            }}
        }}

        {}
        ",
            uma_invitation::FRAGMENT
        );

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("invitation_code", invitation_code.into());
        variables.insert("invitee_uma", invitee_uma.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self
            .requester
            .execute_graphql(&operation, Some(value))
            .await?;

        let result = serde_json::from_value(json["claim_uma_invitation"]["invitation"].clone())
            .map_err(Error::JsonError)?;
        Ok(result)
    }

    /// Claims an UMA invitation as part of the incentive program. If you are not part of the
    /// incentive program, you should use `claim_uma_invitation`.
    pub async fn claim_uma_invitation_with_incentives(
        &self,
        invitation_code: &str,
        invitee_uma: &str,
        invitee_phone_number_e164: &str,
        invitee_region: RegionCode,
    ) -> Result<UmaInvitation, Error> {
        let operation = format!(
            "mutation ClaimUmaInvitation(
            $invitation_code: String!
            $invitee_uma: String!
            $invitee_phone_hash: String!
            $invitee_region: RegionCode!
        ) {{
            claim_uma_invitation_with_incentives(input: {{
                invitation_code: $invitation_code
                invitee_uma: $invitee_uma
                invitee_phone_hash: $invitee_phone_hash
                invitee_region: $invitee_region
            }}) {{
                invitation {{
                    ...UmaInvitationFragment
                }}
            }}
        }}

        {}
        ",
            uma_invitation::FRAGMENT
        );

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("invitation_code", invitation_code.into());
        variables.insert("invitee_uma", invitee_uma.into());
        let invitee_phone_hash = self.hash_phone_number(invitee_phone_number_e164)?;
        variables.insert("invitee_phone_hash", invitee_phone_hash.into());
        variables.insert("invitee_region", invitee_region.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self
            .requester
            .execute_graphql(&operation, Some(value))
            .await?;

        let result = serde_json::from_value(
            json["claim_uma_invitation_with_incentives"]["invitation"].clone(),
        )
        .map_err(Error::JsonError)?;
        Ok(result)
    }

    /// Fetches a UMA invitation by its code.
    pub async fn fetch_uma_invitation(
        &self,
        invitation_code: &str,
    ) -> Result<UmaInvitation, Error> {
        let operation = format!(
            "query FetchUmaInvitation(
            $invitation_code: String!
        ) {{
            uma_invitation_by_code(code: $invitation_code) {{
                ...UmaInvitationFragment
            }}
        }}
        {}
        ",
            uma_invitation::FRAGMENT
        );

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("invitation_code", invitation_code.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self
            .requester
            .execute_graphql(&operation, Some(value))
            .await?;

        let result = serde_json::from_value(json["uma_invitation_by_code"].clone())
            .map_err(Error::JsonError)?;
        Ok(result)
    }

    fn hash_phone_number(phone_number_e164: &str) -> Result<String, Error> {
        let e164_regex = regex::Regex::new(r"^\+[1-9]\d{1,14}$").unwrap();
        if !e164_regex.is_match(phone_number_e164) {
            return Err(Error::InvalidPhoneNumber);
        }
        let mut hasher = Sha256::new();
        hasher.update(phone_number_e164.as_bytes());
        Ok(hex::encode(hasher.finalize()))
    }
}
