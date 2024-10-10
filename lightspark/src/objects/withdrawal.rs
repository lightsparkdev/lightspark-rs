// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::currency_amount::CurrencyAmount;
use crate::objects::entity::Entity;
use crate::objects::on_chain_transaction::OnChainTransaction;
use crate::objects::transaction::Transaction;
use crate::objects::transaction_status::TransactionStatus;
use crate::types::custom_date_formats::custom_date_format;
use crate::types::custom_date_formats::custom_date_format_option;
use crate::types::entity_wrapper::EntityWrapper;
use crate::types::get_entity::GetEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

/// This object represents an L1 withdrawal from your Lightspark Node to any Bitcoin wallet. You can retrieve this object to receive detailed information about any L1 withdrawal associated with your Lightspark Node or account.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Withdrawal {
    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    #[serde(rename = "withdrawal_id")]
    pub id: String,

    /// The date and time when this transaction was initiated.
    #[serde(with = "custom_date_format", rename = "withdrawal_created_at")]
    pub created_at: DateTime<Utc>,

    /// The date and time when the entity was last updated.
    #[serde(with = "custom_date_format", rename = "withdrawal_updated_at")]
    pub updated_at: DateTime<Utc>,

    /// The current status of this transaction.
    #[serde(rename = "withdrawal_status")]
    pub status: TransactionStatus,

    /// The date and time when this transaction was completed or failed.
    #[serde(with = "custom_date_format_option", rename = "withdrawal_resolved_at")]
    pub resolved_at: Option<DateTime<Utc>>,

    /// The amount of money involved in this transaction.
    #[serde(rename = "withdrawal_amount")]
    pub amount: CurrencyAmount,

    /// The hash of this transaction, so it can be uniquely identified on the Lightning Network.
    #[serde(rename = "withdrawal_transaction_hash")]
    pub transaction_hash: Option<String>,

    /// The fees that were paid by the node for this transaction.
    #[serde(rename = "withdrawal_fees")]
    pub fees: Option<CurrencyAmount>,

    /// The hash of the block that included this transaction. This will be null for unconfirmed transactions.
    #[serde(rename = "withdrawal_block_hash")]
    pub block_hash: Option<String>,

    /// The height of the block that included this transaction. This will be zero for unconfirmed transactions.
    #[serde(rename = "withdrawal_block_height")]
    pub block_height: i64,

    /// The Bitcoin blockchain addresses this transaction was sent to.
    #[serde(rename = "withdrawal_destination_addresses")]
    pub destination_addresses: Vec<String>,

    /// The number of blockchain confirmations for this transaction in real time.
    #[serde(rename = "withdrawal_num_confirmations")]
    pub num_confirmations: Option<i64>,

    /// The Lightspark node this withdrawal originated from.
    #[serde(rename = "withdrawal_origin")]
    pub origin: EntityWrapper,

    /// The typename of the object
    #[serde(rename = "__typename")]
    pub typename: String,
}

impl OnChainTransaction for Withdrawal {
    /// The fees that were paid by the node for this transaction.
    fn get_fees(&self) -> Option<CurrencyAmount> {
        self.fees.clone()
    }

    /// The hash of the block that included this transaction. This will be null for unconfirmed transactions.
    fn get_block_hash(&self) -> Option<String> {
        self.block_hash.clone()
    }

    /// The height of the block that included this transaction. This will be zero for unconfirmed transactions.
    fn get_block_height(&self) -> i64 {
        self.block_height
    }

    /// The Bitcoin blockchain addresses this transaction was sent to.
    fn get_destination_addresses(&self) -> Vec<String> {
        self.destination_addresses.clone()
    }

    /// The number of blockchain confirmations for this transaction in real time.
    fn get_num_confirmations(&self) -> Option<i64> {
        self.num_confirmations
    }

    fn type_name(&self) -> &'static str {
        "Withdrawal"
    }
}

impl Transaction for Withdrawal {
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
        "Withdrawal"
    }
}

impl Entity for Withdrawal {
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
        "Withdrawal"
    }
}

impl GetEntity for Withdrawal {
    fn get_entity_query() -> String {
        format!(
            "
        query GetEntity($id: ID!) {{
            entity(id: $id) {{
                ... on Withdrawal {{
                    ... WithdrawalFragment
                }}
            }}
        }}

        {}",
            FRAGMENT
        )
    }
}

pub const FRAGMENT: &str = "
fragment WithdrawalFragment on Withdrawal {
    __typename
    withdrawal_id: id
    withdrawal_created_at: created_at
    withdrawal_updated_at: updated_at
    withdrawal_status: status
    withdrawal_resolved_at: resolved_at
    withdrawal_amount: amount {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    withdrawal_transaction_hash: transaction_hash
    withdrawal_fees: fees {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    withdrawal_block_hash: block_hash
    withdrawal_block_height: block_height
    withdrawal_destination_addresses: destination_addresses
    withdrawal_num_confirmations: num_confirmations
    withdrawal_origin: origin {
        id
    }
}
";
