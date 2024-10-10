
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::currency_amount::CurrencyAmount;
use crate::types::entity_wrapper::EntityWrapper;
use crate::objects::transaction_status::TransactionStatus;
use crate::objects::on_chain_transaction::OnChainTransaction;
use crate::objects::entity::Entity;
use crate::types::get_entity::GetEntity;
use std::vec::Vec;
use crate::types::custom_date_formats::custom_date_format_option;
use crate::objects::transaction::Transaction;
use crate::types::custom_date_formats::custom_date_format;
use chrono::{DateTime, Utc};

/// This object represents a Deposit made to a Lightspark node wallet. This operation occurs for any L1 funding transaction to the wallet. You can retrieve this object to receive detailed information about the deposit.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Deposit {

    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    #[serde (rename = "deposit_id")]
    pub id: String,

    /// The date and time when this transaction was initiated.
    #[serde(with = "custom_date_format", rename = "deposit_created_at")]
    pub created_at: DateTime<Utc>,

    /// The date and time when the entity was last updated.
    #[serde(with = "custom_date_format", rename = "deposit_updated_at")]
    pub updated_at: DateTime<Utc>,

    /// The current status of this transaction.
    #[serde (rename = "deposit_status")]
    pub status: TransactionStatus,

    /// The date and time when this transaction was completed or failed.
    #[serde(with = "custom_date_format_option", rename = "deposit_resolved_at")]
    pub resolved_at: Option<DateTime<Utc>>,

    /// The amount of money involved in this transaction.
    #[serde (rename = "deposit_amount")]
    pub amount: CurrencyAmount,

    /// The hash of this transaction, so it can be uniquely identified on the Lightning Network.
    #[serde (rename = "deposit_transaction_hash")]
    pub transaction_hash: Option<String>,

    /// The fees that were paid by the node for this transaction.
    #[serde (rename = "deposit_fees")]
    pub fees: Option<CurrencyAmount>,

    /// The hash of the block that included this transaction. This will be null for unconfirmed transactions.
    #[serde (rename = "deposit_block_hash")]
    pub block_hash: Option<String>,

    /// The height of the block that included this transaction. This will be zero for unconfirmed transactions.
    #[serde (rename = "deposit_block_height")]
    pub block_height: i64,

    /// The Bitcoin blockchain addresses this transaction was sent to.
    #[serde (rename = "deposit_destination_addresses")]
    pub destination_addresses: Vec<String>,

    /// The number of blockchain confirmations for this transaction in real time.
    #[serde (rename = "deposit_num_confirmations")]
    pub num_confirmations: Option<i64>,

    /// The recipient Lightspark node this deposit was sent to.
    #[serde(rename = "deposit_destination")]
    pub destination: EntityWrapper,

    /// The typename of the object
    #[serde(rename = "__typename")]
    pub typename: String,

}


impl OnChainTransaction for Deposit {

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
        "Deposit"
    }
}



impl Transaction for Deposit {

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
        "Deposit"
    }
}



impl Entity for Deposit {

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
        "Deposit"
    }
}


impl GetEntity for Deposit {
    fn get_entity_query() -> String {
        format!("
        query GetEntity($id: ID!) {{
            entity(id: $id) {{
                ... on Deposit {{
                    ... DepositFragment
                }}
            }}
        }}

        {}", FRAGMENT)
    }    
}



pub const FRAGMENT: &str = "
fragment DepositFragment on Deposit {
    __typename
    deposit_id: id
    deposit_created_at: created_at
    deposit_updated_at: updated_at
    deposit_status: status
    deposit_resolved_at: resolved_at
    deposit_amount: amount {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    deposit_transaction_hash: transaction_hash
    deposit_fees: fees {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    deposit_block_hash: block_hash
    deposit_block_height: block_height
    deposit_destination_addresses: destination_addresses
    deposit_num_confirmations: num_confirmations
    deposit_destination: destination {
        id
    }
}
";



