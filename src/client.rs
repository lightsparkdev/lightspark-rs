// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use std::cell::RefCell;
use std::collections::HashMap;

use serde_json::Value;

use crate::crypto::decrypt_private_key;
use crate::error::Error;
use crate::objects::account::Account;
use crate::objects::api_token::ApiToken;
use crate::objects::currency_amount::{self, CurrencyAmount};
use crate::objects::fee_estimate::FeeEstimate;
use crate::objects::invoice;
use crate::objects::invoice::Invoice;
use crate::objects::invoice_data::InvoiceData;
use crate::objects::lightning_fee_estimate_output::LightningFeeEstimateOutput;
use crate::objects::outgoing_payment::OutgoingPayment;
use crate::objects::permission::Permission;
use crate::objects::withdrawal_mode::WithdrawalMode;
use crate::objects::withdrawal_request::WithdrawalRequest;
use crate::objects::{account, invoice_data};
use crate::objects::{api_token, outgoing_payment};
use crate::objects::{bitcoin_network, withdrawal_request};
use crate::objects::{fee_estimate, lightning_fee_estimate_output};
use crate::requester::auth_provider::AuthProvider;
use crate::requester::requester::{Requester, RequesterError};
use crate::types::get_entity::GetEntity;

pub struct LightsparkClient {
    pub requester: Requester,
    signing_keys: RefCell<HashMap<String, Vec<u8>>>,
}

impl LightsparkClient {
    pub fn new<T: AuthProvider>(auth_provider: T) -> Result<Self, Error> {
        let requester = Requester::new(auth_provider)?;
        Ok(LightsparkClient {
            requester,
            signing_keys: RefCell::new(HashMap::new()),
        })
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
        let json = self
            .requester
            .execute_graphql(&query, Some(value))
            .await
            .map_err(Error::ClientError)?;
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
        let json = self
            .requester
            .execute_graphql(&query, Some(value))
            .await
            .map_err(Error::ClientError)?;
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
        let json = self
            .requester
            .execute_graphql(&query, Some(value))
            .await
            .map_err(Error::ClientError)?;
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

        let json = self
            .requester
            .execute_graphql(&query, None)
            .await
            .map_err(Error::ClientError)?;
        let result =
            serde_json::from_value(json["current_account"].clone()).map_err(Error::JsonError)?;
        Ok(result)
    }

    pub async fn create_api_token(
        &self,
        name: &str,
        permission: Option<Vec<Permission>>,
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
        if permission.is_none() {
            variables.insert("permissions", vec![Permission::All].into());
        } else {
            variables.insert("permissions", permission.into());
        }

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self
            .requester
            .execute_graphql(&operation, Some(value))
            .await
            .map_err(Error::ClientError)?;

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
            .await
            .map_err(Error::ClientError)?;
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
            .await
            .map_err(Error::ClientError)?;

        let result = serde_json::from_value(json["entity"].clone()).map_err(Error::JsonError)?;
        Ok(result)
    }

    pub async fn create_invoice(
        &self,
        node_id: &str,
        amount_msats: i64,
        memo: Option<&str>,
    ) -> Result<Invoice, Error> {
        let operation = format!(
            "mutation CreateInvoice(
                $node_id: ID!
                $amount_msats: Long!
                $memo: String
            ) {{
                create_invoice(input: {{
                    node_id: $node_id
                    amount_msats: $amount_msats
                    memo: $memo
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

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self
            .requester
            .execute_graphql(&operation, Some(value))
            .await
            .map_err(Error::ClientError)?;

        let result = serde_json::from_value(json["create_invoice"]["invoice"].clone())
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
            .await
            .map_err(Error::ClientError)?;

        let result = serde_json::from_value(json["fund_node"]["amount"].clone())
            .map_err(Error::JsonError)?;
        Ok(result)
    }

    fn load_node_signing_key(&self, node_id: &str, signing_key: Vec<u8>) {
        self.signing_keys
            .borrow_mut()
            .insert(node_id.to_owned(), signing_key);
    }

    fn get_node_signing_key(&self, node_id: &str) -> Option<Vec<u8>> {
        return self.signing_keys.borrow_mut().get(node_id).cloned();
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
            .await
            .map_err(Error::ClientError)?;

        let result = serde_json::from_value(json["decoded_payment_request"].clone())
            .map_err(Error::JsonError)?;
        Ok(result)
    }

    pub async fn recover_node_signing_key(
        &self,
        node_id: &str,
        node_password: &str,
    ) -> Result<Vec<u8>, Error> {
        let operation = "query RecoverNodeSigningKey(
                $node_id: ID!
            ) {
                entity(id: $node_id) {
                    ... on LightsparkNode {
                        encrypted_signing_private_key {
                            encrypted_value
                            cipher
                        }
                    }
                }
            }"
        .to_string();

        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("node_id", node_id.clone().into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self
            .requester
            .execute_graphql(&operation, Some(value))
            .await
            .map_err(Error::ClientError)?;

        let encrypted_key = json["entity"]["encrypted_signing_private_key"]["encrypted_value"]
            .as_str()
            .ok_or(Error::ClientError(RequesterError::GraphqlError(
                "missing encrypted_value".to_owned(),
            )))?;
        let cipher = json["entity"]["encrypted_signing_private_key"]["cipher"]
            .as_str()
            .ok_or(Error::ClientError(RequesterError::GraphqlError(
                "missing cipher".to_owned(),
            )))?;

        let decrypted_private_key = decrypt_private_key(cipher, encrypted_key, node_password)
            .map_err(Error::CryptoError)?;
        self.load_node_signing_key(node_id.clone(), decrypted_private_key.clone());
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
        variables.insert("node_id", node_id.clone().into());
        variables.insert("encoded_invoice", encoded_invoice.clone().into());
        variables.insert("amount_msats", amount_msats.into());
        variables.insert("timeout_secs", timeout_secs.into());
        variables.insert("maximum_fees_msats", maximum_fees_msats.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;

        let signing_key = self.get_node_signing_key(node_id);
        let json = self
            .requester
            .execute_graphql_signing(&operation, Some(value), signing_key)
            .await
            .map_err(Error::ClientError)?;

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
        variables.insert("node_id", node_id.clone().into());
        variables.insert(
            "destination_public_key",
            destination_public_key.clone().into(),
        );
        variables.insert("amount_msats", amount_msats.into());
        variables.insert("timeout_secs", timeout_secs.into());
        variables.insert("maximum_fees_msats", maximum_fees_msats.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;

        let signing_key = self.get_node_signing_key(node_id);

        let json = self
            .requester
            .execute_graphql_signing(&operation, Some(value), signing_key)
            .await
            .map_err(Error::ClientError)?;

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
            .execute_graphql(&operation, Some(value))
            .await
            .map_err(Error::ClientError)?;
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
        variables.insert("node_id", node_id.clone().into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let json = self
            .requester
            .execute_graphql(&operation, Some(value))
            .await
            .map_err(Error::ClientError)?;

        if let Some(result) = json["create_node_wallet_address"]["wallet_address"].as_str() {
            Ok(result.to_owned())
        } else {
            Err(Error::ClientError(RequesterError::GraphqlError(
                "missing wallet_address".to_owned(),
            )))
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
            $amount_sats: Int!
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
        variables.insert("node_id", node_id.clone().into());
        variables.insert("amount_sats", amount_sats.into());
        variables.insert("bitcoin_address", bitcoin_address.clone().into());
        variables.insert("withdrawal_mode", withdrawal_mode.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let signing_key = self.get_node_signing_key(node_id);
        let json = self
            .requester
            .execute_graphql_signing(&operation, Some(value), signing_key)
            .await
            .map_err(Error::ClientError)?;

        println!("{}", json);
        let result = serde_json::from_value(json["request_withdrawal"]["request"].clone())
            .map_err(Error::JsonError)?;
        Ok(result)
    }
}
