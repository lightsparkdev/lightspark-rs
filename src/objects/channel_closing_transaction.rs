// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::currency_amount::CurrencyAmount;
use crate::objects::entity::Entity;
use crate::objects::on_chain_transaction::OnChainTransaction;
use crate::objects::transaction::Transaction;
use crate::objects::transaction_status::TransactionStatus;
use crate::types::custom_date_format::custom_date_format;
use crate::types::custom_date_format::custom_date_format_option;
use crate::types::entity_wrapper::EntityWrapper;
use crate::types::get_entity::GetEntity;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::vec::Vec;

/// The transaction on Bitcoin blockchain to close a channel on Lightning Network where the balances are allocated back to local and remote nodes.
#[derive(Deserialize)]
pub struct ChannelClosingTransaction {
    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    #[serde(rename = "channel_closing_transaction_id")]
    pub id: String,

    /// The date and time when this transaction was initiated.
    #[serde(
        with = "custom_date_format",
        rename = "channel_closing_transaction_created_at"
    )]
    pub created_at: DateTime<Utc>,

    /// The date and time when the entity was last updated.
    #[serde(
        with = "custom_date_format",
        rename = "channel_closing_transaction_updated_at"
    )]
    pub updated_at: DateTime<Utc>,

    /// The current status of this transaction.
    #[serde(rename = "channel_closing_transaction_status")]
    pub status: TransactionStatus,

    /// The date and time when this transaction was completed or failed.
    #[serde(
        with = "custom_date_format_option",
        rename = "channel_closing_transaction_resolved_at"
    )]
    pub resolved_at: Option<DateTime<Utc>>,

    /// The amount of money involved in this transaction.
    #[serde(rename = "channel_closing_transaction_amount")]
    pub amount: CurrencyAmount,

    /// The hash of this transaction, so it can be uniquely identified on the Lightning Network.
    #[serde(rename = "channel_closing_transaction_transaction_hash")]
    pub transaction_hash: Option<String>,

    /// The fees that were paid by the wallet sending the transaction to commit it to the Bitcoin blockchain.
    #[serde(rename = "channel_closing_transaction_fees")]
    pub fees: Option<CurrencyAmount>,

    /// The hash of the block that included this transaction. This will be null for unconfirmed transactions.
    #[serde(rename = "channel_closing_transaction_block_hash")]
    pub block_hash: Option<String>,

    /// The height of the block that included this transaction. This will be zero for unconfirmed transactions.
    #[serde(rename = "channel_closing_transaction_block_height")]
    pub block_height: i64,

    /// The Bitcoin blockchain addresses this transaction was sent to.
    #[serde(rename = "channel_closing_transaction_destination_addresses")]
    pub destination_addresses: Vec<String>,

    /// The number of blockchain confirmations for this transaction in real time.
    #[serde(rename = "channel_closing_transaction_num_confirmations")]
    pub num_confirmations: Option<i64>,

    /// If known, the channel this transaction is closing.
    #[serde(rename = "channel_closing_transaction_channel")]
    pub channel: Option<EntityWrapper>,
}

impl OnChainTransaction for ChannelClosingTransaction {
    /// The fees that were paid by the wallet sending the transaction to commit it to the Bitcoin blockchain.
    fn get_fees(&self) -> Option<CurrencyAmount> {
        return self.fees.clone();
    }

    /// The hash of the block that included this transaction. This will be null for unconfirmed transactions.
    fn get_block_hash(&self) -> Option<String> {
        return self.block_hash.clone();
    }

    /// The height of the block that included this transaction. This will be zero for unconfirmed transactions.
    fn get_block_height(&self) -> i64 {
        return self.block_height;
    }

    /// The Bitcoin blockchain addresses this transaction was sent to.
    fn get_destination_addresses(&self) -> Vec<String> {
        return self.destination_addresses.clone();
    }

    /// The number of blockchain confirmations for this transaction in real time.
    fn get_num_confirmations(&self) -> Option<i64> {
        return self.num_confirmations;
    }

    fn type_name(&self) -> &'static str {
        "ChannelClosingTransaction"
    }
}

impl Transaction for ChannelClosingTransaction {
    /// The current status of this transaction.
    fn get_status(&self) -> TransactionStatus {
        return self.status.clone();
    }

    /// The date and time when this transaction was completed or failed.
    fn get_resolved_at(&self) -> Option<DateTime<Utc>> {
        return self.resolved_at;
    }

    /// The amount of money involved in this transaction.
    fn get_amount(&self) -> CurrencyAmount {
        return self.amount.clone();
    }

    /// The hash of this transaction, so it can be uniquely identified on the Lightning Network.
    fn get_transaction_hash(&self) -> Option<String> {
        return self.transaction_hash.clone();
    }

    fn type_name(&self) -> &'static str {
        "ChannelClosingTransaction"
    }
}

impl Entity for ChannelClosingTransaction {
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
        "ChannelClosingTransaction"
    }
}

impl GetEntity for ChannelClosingTransaction {
    fn get_entity_query() -> String {
        return format!(
            "
        query GetEntity($id: ID!) {{
            entity(id: $id) {{
                ... on ChannelClosingTransaction {{
                    ... ChannelClosingTransactionFragment
                }}
            }}
        }}

        {}",
            FRAGMENT
        );
    }
}

pub const FRAGMENT: &str = "
fragment ChannelClosingTransactionFragment on ChannelClosingTransaction {
    __typename
    channel_closing_transaction_id: id
    channel_closing_transaction_created_at: created_at
    channel_closing_transaction_updated_at: updated_at
    channel_closing_transaction_status: status
    channel_closing_transaction_resolved_at: resolved_at
    channel_closing_transaction_amount: amount {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    channel_closing_transaction_transaction_hash: transaction_hash
    channel_closing_transaction_fees: fees {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    channel_closing_transaction_block_hash: block_hash
    channel_closing_transaction_block_height: block_height
    channel_closing_transaction_destination_addresses: destination_addresses
    channel_closing_transaction_num_confirmations: num_confirmations
    channel_closing_transaction_channel: channel {
        id
    }
}
";
