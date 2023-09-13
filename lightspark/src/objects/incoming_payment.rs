// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::error::Error;
use crate::objects::currency_amount::CurrencyAmount;
use crate::objects::entity::Entity;
use crate::objects::incoming_payment_attempt_status::IncomingPaymentAttemptStatus;
use crate::objects::incoming_payment_to_attempts_connection::IncomingPaymentToAttemptsConnection;
use crate::objects::lightning_transaction::LightningTransaction;
use crate::objects::post_transaction_data::PostTransactionData;
use crate::objects::transaction::Transaction;
use crate::objects::transaction_status::TransactionStatus;
use crate::request::requester::Requester;
use crate::types::custom_date_formats::custom_date_format;
use crate::types::custom_date_formats::custom_date_format_option;
use crate::types::entity_wrapper::EntityWrapper;
use crate::types::get_entity::GetEntity;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::vec::Vec;

/// This object represents any payment sent to a Lightspark node on the Lightning Network. You can retrieve this object to receive payment related information about a specific payment received by a Lightspark node.
#[derive(Clone, Deserialize)]
pub struct IncomingPayment {
    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    #[serde(rename = "incoming_payment_id")]
    pub id: String,

    /// The date and time when this transaction was initiated.
    #[serde(with = "custom_date_format", rename = "incoming_payment_created_at")]
    pub created_at: DateTime<Utc>,

    /// The date and time when the entity was last updated.
    #[serde(with = "custom_date_format", rename = "incoming_payment_updated_at")]
    pub updated_at: DateTime<Utc>,

    /// The current status of this transaction.
    #[serde(rename = "incoming_payment_status")]
    pub status: TransactionStatus,

    /// The date and time when this transaction was completed or failed.
    #[serde(
        with = "custom_date_format_option",
        rename = "incoming_payment_resolved_at"
    )]
    pub resolved_at: Option<DateTime<Utc>>,

    /// The amount of money involved in this transaction.
    #[serde(rename = "incoming_payment_amount")]
    pub amount: CurrencyAmount,

    /// The hash of this transaction, so it can be uniquely identified on the Lightning Network.
    #[serde(rename = "incoming_payment_transaction_hash")]
    pub transaction_hash: Option<String>,

    /// The recipient Lightspark node this payment was sent to.
    #[serde(rename = "incoming_payment_destination")]
    pub destination: EntityWrapper,

    /// The optional payment request for this incoming payment, which will be null if the payment is sent through keysend.
    #[serde(rename = "incoming_payment_payment_request")]
    pub payment_request: Option<EntityWrapper>,

    /// The post transaction data which can be used in KYT payment registration.
    #[serde(rename = "incoming_payment_uma_post_transaction_data")]
    pub uma_post_transaction_data: Option<Vec<PostTransactionData>>,
}

impl LightningTransaction for IncomingPayment {
    fn type_name(&self) -> &'static str {
        "IncomingPayment"
    }
}

impl Transaction for IncomingPayment {
    /// The current status of this transaction.
    fn get_status(&self) -> TransactionStatus {
        self.status.clone()
    }

    /// The date and time when this transaction was completed or failed.
    fn get_resolved_at(&self) -> Option<DateTime<Utc>> {
        self.resolved_at
    }

    /// The amount of money involved in this transaction.
    fn get_amount(&self) -> CurrencyAmount {
        self.amount.clone()
    }

    /// The hash of this transaction, so it can be uniquely identified on the Lightning Network.
    fn get_transaction_hash(&self) -> Option<String> {
        self.transaction_hash.clone()
    }

    fn type_name(&self) -> &'static str {
        "IncomingPayment"
    }
}

impl Entity for IncomingPayment {
    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    fn get_id(&self) -> String {
        self.id.clone()
    }

    /// The date and time when the entity was first created.
    fn get_created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// The date and time when the entity was last updated.
    fn get_updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    fn type_name(&self) -> &'static str {
        "IncomingPayment"
    }
}

impl GetEntity for IncomingPayment {
    fn get_entity_query() -> String {
        format!(
            "
        query GetEntity($id: ID!) {{
            entity(id: $id) {{
                ... on IncomingPayment {{
                    ... IncomingPaymentFragment
                }}
            }}
        }}

        {}",
            FRAGMENT
        )
    }
}

pub const FRAGMENT: &str = "
fragment IncomingPaymentFragment on IncomingPayment {
    __typename
    incoming_payment_id: id
    incoming_payment_created_at: created_at
    incoming_payment_updated_at: updated_at
    incoming_payment_status: status
    incoming_payment_resolved_at: resolved_at
    incoming_payment_amount: amount {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    incoming_payment_transaction_hash: transaction_hash
    incoming_payment_destination: destination {
        id
    }
    incoming_payment_payment_request: payment_request {
        id
    }
    incoming_payment_uma_post_transaction_data: uma_post_transaction_data {
        __typename
        post_transaction_data_utxo: utxo
        post_transaction_data_amount: amount {
            __typename
            currency_amount_original_value: original_value
            currency_amount_original_unit: original_unit
            currency_amount_preferred_currency_unit: preferred_currency_unit
            currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
            currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
        }
    }
}
";

impl IncomingPayment {
    pub async fn get_attempts(
        &self,
        requester: &Requester,
        first: Option<i64>,
        statuses: Option<Vec<IncomingPaymentAttemptStatus>>,
        after: Option<String>,
    ) -> Result<IncomingPaymentToAttemptsConnection, Error> {
        let query = "query FetchIncomingPaymentToAttemptsConnection($entity_id: ID!, $first: Int, $statuses: [IncomingPaymentAttemptStatus!], $after: String) {
    entity(id: $entity_id) {
        ... on IncomingPayment {
            attempts(, first: $first, statuses: $statuses, after: $after) {
                __typename
                incoming_payment_to_attempts_connection_count: count
                incoming_payment_to_attempts_connection_page_info: page_info {
                    __typename
                    page_info_has_next_page: has_next_page
                    page_info_has_previous_page: has_previous_page
                    page_info_start_cursor: start_cursor
                    page_info_end_cursor: end_cursor
                }
                incoming_payment_to_attempts_connection_entities: entities {
                    __typename
                    incoming_payment_attempt_id: id
                    incoming_payment_attempt_created_at: created_at
                    incoming_payment_attempt_updated_at: updated_at
                    incoming_payment_attempt_status: status
                    incoming_payment_attempt_resolved_at: resolved_at
                    incoming_payment_attempt_amount: amount {
                        __typename
                        currency_amount_original_value: original_value
                        currency_amount_original_unit: original_unit
                        currency_amount_preferred_currency_unit: preferred_currency_unit
                        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
                        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
                    }
                    incoming_payment_attempt_channel: channel {
                        id
                    }
                }
            }
        }
    }
}";
        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("entity_id", self.id.clone().into());
        variables.insert("first", first.into());
        variables.insert("statuses", statuses.into());
        variables.insert("after", after.into());

        let value = serde_json::to_value(variables).map_err(Error::ConversionError)?;
        let result = requester
            .execute_graphql(query, Some(value))
            .await
            .map_err(Error::ClientError)?;
        let json = result["entity"]["attempts"].clone();
        let result = serde_json::from_value(json).map_err(Error::JsonError)?;
        Ok(result)
    }
}
