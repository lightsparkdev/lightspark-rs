// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::currency_amount::CurrencyAmount;
use crate::objects::entity::Entity;
use crate::objects::lightning_transaction::LightningTransaction;
use crate::objects::rich_text::RichText;
use crate::objects::routing_transaction_failure_reason::RoutingTransactionFailureReason;
use crate::objects::transaction::Transaction;
use crate::objects::transaction_status::TransactionStatus;
use crate::types::custom_date_formats::custom_date_format;
use crate::types::custom_date_formats::custom_date_format_option;
use crate::types::entity_wrapper::EntityWrapper;
use crate::types::get_entity::GetEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// This object represents a transaction that was forwarded through a Lightspark node on the Lightning Network, i.e., a routed transaction. You can retrieve this object to receive information about any transaction routed through your Lightspark Node.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoutingTransaction {
    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    #[serde(rename = "routing_transaction_id")]
    pub id: String,

    /// The date and time when this transaction was initiated.
    #[serde(with = "custom_date_format", rename = "routing_transaction_created_at")]
    pub created_at: DateTime<Utc>,

    /// The date and time when the entity was last updated.
    #[serde(with = "custom_date_format", rename = "routing_transaction_updated_at")]
    pub updated_at: DateTime<Utc>,

    /// The current status of this transaction.
    #[serde(rename = "routing_transaction_status")]
    pub status: TransactionStatus,

    /// The date and time when this transaction was completed or failed.
    #[serde(
        with = "custom_date_format_option",
        rename = "routing_transaction_resolved_at"
    )]
    pub resolved_at: Option<DateTime<Utc>>,

    /// The amount of money involved in this transaction.
    #[serde(rename = "routing_transaction_amount")]
    pub amount: CurrencyAmount,

    /// The hash of this transaction, so it can be uniquely identified on the Lightning Network.
    #[serde(rename = "routing_transaction_transaction_hash")]
    pub transaction_hash: Option<String>,

    /// If known, the channel this transaction was received from.
    #[serde(rename = "routing_transaction_incoming_channel")]
    pub incoming_channel: Option<EntityWrapper>,

    /// If known, the channel this transaction was forwarded to.
    #[serde(rename = "routing_transaction_outgoing_channel")]
    pub outgoing_channel: Option<EntityWrapper>,

    /// The fees collected by the node when routing this transaction. We subtract the outgoing amount to the incoming amount to determine how much fees were collected.
    #[serde(rename = "routing_transaction_fees")]
    pub fees: Option<CurrencyAmount>,

    /// If applicable, user-facing error message describing why the routing failed.
    #[serde(rename = "routing_transaction_failure_message")]
    pub failure_message: Option<RichText>,

    /// If applicable, the reason why the routing failed.
    #[serde(rename = "routing_transaction_failure_reason")]
    pub failure_reason: Option<RoutingTransactionFailureReason>,

    /// The typename of the object
    #[serde(rename = "__typename")]
    pub typename: String,
}

impl LightningTransaction for RoutingTransaction {
    fn type_name(&self) -> &'static str {
        "RoutingTransaction"
    }
}

impl Transaction for RoutingTransaction {
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
        "RoutingTransaction"
    }
}

impl Entity for RoutingTransaction {
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
        "RoutingTransaction"
    }
}

impl GetEntity for RoutingTransaction {
    fn get_entity_query() -> String {
        format!(
            "
        query GetEntity($id: ID!) {{
            entity(id: $id) {{
                ... on RoutingTransaction {{
                    ... RoutingTransactionFragment
                }}
            }}
        }}

        {}",
            FRAGMENT
        )
    }
}

pub const FRAGMENT: &str = "
fragment RoutingTransactionFragment on RoutingTransaction {
    __typename
    routing_transaction_id: id
    routing_transaction_created_at: created_at
    routing_transaction_updated_at: updated_at
    routing_transaction_status: status
    routing_transaction_resolved_at: resolved_at
    routing_transaction_amount: amount {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    routing_transaction_transaction_hash: transaction_hash
    routing_transaction_incoming_channel: incoming_channel {
        id
    }
    routing_transaction_outgoing_channel: outgoing_channel {
        id
    }
    routing_transaction_fees: fees {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    routing_transaction_failure_message: failure_message {
        __typename
        rich_text_text: text
    }
    routing_transaction_failure_reason: failure_reason
}
";
