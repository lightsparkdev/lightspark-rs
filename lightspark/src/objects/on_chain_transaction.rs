// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use super::channel_closing_transaction::ChannelClosingTransaction;
use super::channel_opening_transaction::ChannelOpeningTransaction;
use super::deposit::Deposit;
use super::withdrawal::Withdrawal;
use crate::objects::currency_amount::CurrencyAmount;
use crate::objects::entity::Entity;
use crate::objects::transaction::Transaction;
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::vec::Vec;

pub trait OnChainTransaction: Transaction + Entity {
    /// The fees that were paid by the wallet sending the transaction to commit it to the Bitcoin blockchain.
    fn get_fees(&self) -> Option<CurrencyAmount>;

    /// The hash of the block that included this transaction. This will be null for unconfirmed transactions.
    fn get_block_hash(&self) -> Option<String>;

    /// The height of the block that included this transaction. This will be zero for unconfirmed transactions.
    fn get_block_height(&self) -> i64;

    /// The Bitcoin blockchain addresses this transaction was sent to.
    fn get_destination_addresses(&self) -> Vec<String>;

    /// The number of blockchain confirmations for this transaction in real time.
    fn get_num_confirmations(&self) -> Option<i64>;

    fn type_name(&self) -> &'static str;
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum OnChainTransactionEnum {
    ChannelClosingTransaction(ChannelClosingTransaction),
    ChannelOpeningTransaction(ChannelOpeningTransaction),
    Deposit(Deposit),
    Withdrawal(Withdrawal),
}

impl<'de> Deserialize<'de> for OnChainTransactionEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        if let Some(typename) = value.get("__typename").and_then(Value::as_str) {
            match typename {
                "ChannelClosingTransaction" => {
                    let obj = ChannelClosingTransaction::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(OnChainTransactionEnum::ChannelClosingTransaction(obj))
                }
                "ChannelOpeningTransaction" => {
                    let obj = ChannelOpeningTransaction::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(OnChainTransactionEnum::ChannelOpeningTransaction(obj))
                }
                "Deposit" => {
                    let obj = Deposit::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(OnChainTransactionEnum::Deposit(obj))
                }
                "Withdrawal" => {
                    let obj = Withdrawal::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(OnChainTransactionEnum::Withdrawal(obj))
                }

                _ => Err(serde::de::Error::custom("unknown typename")),
            }
        } else {
            Err(serde::de::Error::custom(
                "missing __typename field on OnChainTransaction",
            ))
        }
    }
}
