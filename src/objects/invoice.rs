// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::currency_amount::CurrencyAmount;
use crate::objects::entity::Entity;
use crate::objects::invoice_data::InvoiceData;
use crate::objects::payment_request::PaymentRequest;
use crate::objects::payment_request_data::PaymentRequestData;
use crate::objects::payment_request_status::PaymentRequestStatus;
use crate::types::custom_date_format::custom_date_format;
use crate::types::get_entity::GetEntity;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// This object represents a BOLT #11 invoice (https://github.com/lightning/bolts/blob/master/11-payment-encoding.md) initiated by a Lightspark Node.
#[derive(Deserialize)]
pub struct Invoice {
    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    #[serde(rename = "invoice_id")]
    pub id: String,

    /// The date and time when the entity was first created.
    #[serde(with = "custom_date_format", rename = "invoice_created_at")]
    pub created_at: DateTime<Utc>,

    /// The date and time when the entity was last updated.
    #[serde(with = "custom_date_format", rename = "invoice_updated_at")]
    pub updated_at: DateTime<Utc>,

    /// The details of the invoice.
    #[serde(rename = "invoice_data")]
    pub data: InvoiceData,

    /// The status of the payment request.
    #[serde(rename = "invoice_status")]
    pub status: PaymentRequestStatus,

    /// The total amount that has been paid to this invoice.
    #[serde(rename = "invoice_amount_paid")]
    pub amount_paid: Option<CurrencyAmount>,
}

impl PaymentRequest for Invoice {
    /// The details of the payment request.
    fn get_data(&self) -> &dyn PaymentRequestData {
        return &self.data;
    }

    /// The status of the payment request.
    fn get_status(&self) -> PaymentRequestStatus {
        return self.status.clone();
    }

    fn type_name(&self) -> &'static str {
        "Invoice"
    }
}

impl Entity for Invoice {
    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    fn get_id(&self) -> String {
        return self.id.clone();
    }

    /// The date and time when the entity was first created.
    fn get_created_at(&self) -> DateTime<Utc> {
        return self.created_at;
    }

    /// The date and time when the entity was last updated.
    fn get_updated_at(&self) -> DateTime<Utc> {
        return self.updated_at;
    }

    fn type_name(&self) -> &'static str {
        "Invoice"
    }
}

impl GetEntity for Invoice {
    fn get_entity_query() -> String {
        return format!(
            "
        query GetEntity($id: ID!) {{
            entity(id: $id) {{
                ... on Invoice {{
                    ... InvoiceFragment
                }}
            }}
        }}

        {}",
            FRAGMENT
        );
    }
}

pub const FRAGMENT: &str = "
fragment InvoiceFragment on Invoice {
    __typename
    invoice_id: id
    invoice_created_at: created_at
    invoice_updated_at: updated_at
    invoice_data: data {
        __typename
        invoice_data_encoded_payment_request: encoded_payment_request
        invoice_data_bitcoin_network: bitcoin_network
        invoice_data_payment_hash: payment_hash
        invoice_data_amount: amount {
            __typename
            currency_amount_original_value: original_value
            currency_amount_original_unit: original_unit
            currency_amount_preferred_currency_unit: preferred_currency_unit
            currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
            currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
        }
        invoice_data_created_at: created_at
        invoice_data_expires_at: expires_at
        invoice_data_destination: destination {
            __typename
            ... on GraphNode {
                __typename
                graph_node_id: id
                graph_node_created_at: created_at
                graph_node_updated_at: updated_at
                graph_node_alias: alias
                graph_node_bitcoin_network: bitcoin_network
                graph_node_color: color
                graph_node_conductivity: conductivity
                graph_node_display_name: display_name
                graph_node_public_key: public_key
            }
            ... on LightsparkNode {
                __typename
                lightspark_node_id: id
                lightspark_node_created_at: created_at
                lightspark_node_updated_at: updated_at
                lightspark_node_alias: alias
                lightspark_node_bitcoin_network: bitcoin_network
                lightspark_node_color: color
                lightspark_node_conductivity: conductivity
                lightspark_node_display_name: display_name
                lightspark_node_public_key: public_key
                lightspark_node_account: account {
                    id
                }
                lightspark_node_blockchain_balance: blockchain_balance {
                    __typename
                    blockchain_balance_total_balance: total_balance {
                        __typename
                        currency_amount_original_value: original_value
                        currency_amount_original_unit: original_unit
                        currency_amount_preferred_currency_unit: preferred_currency_unit
                        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
                        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
                    }
                    blockchain_balance_confirmed_balance: confirmed_balance {
                        __typename
                        currency_amount_original_value: original_value
                        currency_amount_original_unit: original_unit
                        currency_amount_preferred_currency_unit: preferred_currency_unit
                        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
                        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
                    }
                    blockchain_balance_unconfirmed_balance: unconfirmed_balance {
                        __typename
                        currency_amount_original_value: original_value
                        currency_amount_original_unit: original_unit
                        currency_amount_preferred_currency_unit: preferred_currency_unit
                        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
                        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
                    }
                    blockchain_balance_locked_balance: locked_balance {
                        __typename
                        currency_amount_original_value: original_value
                        currency_amount_original_unit: original_unit
                        currency_amount_preferred_currency_unit: preferred_currency_unit
                        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
                        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
                    }
                    blockchain_balance_required_reserve: required_reserve {
                        __typename
                        currency_amount_original_value: original_value
                        currency_amount_original_unit: original_unit
                        currency_amount_preferred_currency_unit: preferred_currency_unit
                        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
                        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
                    }
                    blockchain_balance_available_balance: available_balance {
                        __typename
                        currency_amount_original_value: original_value
                        currency_amount_original_unit: original_unit
                        currency_amount_preferred_currency_unit: preferred_currency_unit
                        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
                        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
                    }
                }
                lightspark_node_encrypted_signing_private_key: encrypted_signing_private_key {
                    __typename
                    secret_encrypted_value: encrypted_value
                    secret_cipher: cipher
                }
                lightspark_node_total_balance: total_balance {
                    __typename
                    currency_amount_original_value: original_value
                    currency_amount_original_unit: original_unit
                    currency_amount_preferred_currency_unit: preferred_currency_unit
                    currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
                    currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
                }
                lightspark_node_total_local_balance: total_local_balance {
                    __typename
                    currency_amount_original_value: original_value
                    currency_amount_original_unit: original_unit
                    currency_amount_preferred_currency_unit: preferred_currency_unit
                    currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
                    currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
                }
                lightspark_node_local_balance: local_balance {
                    __typename
                    currency_amount_original_value: original_value
                    currency_amount_original_unit: original_unit
                    currency_amount_preferred_currency_unit: preferred_currency_unit
                    currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
                    currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
                }
                lightspark_node_purpose: purpose
                lightspark_node_remote_balance: remote_balance {
                    __typename
                    currency_amount_original_value: original_value
                    currency_amount_original_unit: original_unit
                    currency_amount_preferred_currency_unit: preferred_currency_unit
                    currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
                    currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
                }
                lightspark_node_status: status
            }
        }
        invoice_data_memo: memo
    }
    invoice_status: status
    invoice_amount_paid: amount_paid {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
}
";
