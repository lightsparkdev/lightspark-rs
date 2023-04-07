// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use super::channel_closing_transaction::ChannelClosingTransaction;
use super::channel_opening_transaction::ChannelOpeningTransaction;
use super::deposit::Deposit;
use super::incoming_payment::IncomingPayment;
use super::outgoing_payment::OutgoingPayment;
use super::routing_transaction::RoutingTransaction;
use super::withdrawal::Withdrawal;
use crate::objects::currency_amount::CurrencyAmount;
use crate::objects::entity::Entity;
use crate::objects::transaction_status::TransactionStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub trait Transaction: Entity {
    /// The current status of this transaction.
    fn get_status(&self) -> TransactionStatus;

    /// The date and time when this transaction was completed or failed.
    fn get_resolved_at(&self) -> Option<DateTime<Utc>>;

    /// The amount of money involved in this transaction.
    fn get_amount(&self) -> CurrencyAmount;

    /// The hash of this transaction, so it can be uniquely identified on the Lightning Network.
    fn get_transaction_hash(&self) -> Option<String>;

    fn type_name(&self) -> &'static str;
}

impl<'de> Deserialize<'de> for Box<dyn Transaction> {
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
                    Ok(Box::new(obj))
                }
                "ChannelOpeningTransaction" => {
                    let obj = ChannelOpeningTransaction::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(Box::new(obj))
                }
                "Deposit" => {
                    let obj = Deposit::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(Box::new(obj))
                }
                "IncomingPayment" => {
                    let obj = IncomingPayment::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(Box::new(obj))
                }
                "OutgoingPayment" => {
                    let obj = OutgoingPayment::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(Box::new(obj))
                }
                "RoutingTransaction" => {
                    let obj = RoutingTransaction::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(Box::new(obj))
                }
                "Withdrawal" => {
                    let obj = Withdrawal::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(Box::new(obj))
                }

                _ => Err(serde::de::Error::custom("unknown typename")),
            }
        } else {
            Err(serde::de::Error::custom(
                "missing __typename field on Transaction",
            ))
        }
    }
}
