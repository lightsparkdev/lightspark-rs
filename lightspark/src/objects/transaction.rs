
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::entity::Entity;
use crate::objects::currency_amount::CurrencyAmount;
use serde_json::Value;
use super::routing_transaction::RoutingTransaction;
use super::withdrawal::Withdrawal;
use super::channel_opening_transaction::ChannelOpeningTransaction;
use super::deposit::Deposit;
use super::outgoing_payment::OutgoingPayment;
use super::channel_closing_transaction::ChannelClosingTransaction;
use super::incoming_payment::IncomingPayment;
use serde::{Deserialize, Deserializer, Serialize};
use crate::objects::transaction_status::TransactionStatus;
use chrono::{DateTime, Utc};

pub trait Transaction : Entity {

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


#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize)]
pub enum TransactionEnum {
    ChannelClosingTransaction(ChannelClosingTransaction),
    ChannelOpeningTransaction(ChannelOpeningTransaction),
    Deposit(Deposit),
    IncomingPayment(IncomingPayment),
    OutgoingPayment(OutgoingPayment),
    RoutingTransaction(RoutingTransaction),
    Withdrawal(Withdrawal),

}



impl<'de> Deserialize<'de> for TransactionEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        if let Some(typename) = value.get("__typename").and_then(Value::as_str) {
            match typename {
                "ChannelClosingTransaction" => {
                    let obj = ChannelClosingTransaction::deserialize(value)
                        .map_err(|err|
                            serde::de::Error::custom(format!("Serde JSON Error {}", err))
                        )?;
                    Ok(TransactionEnum::ChannelClosingTransaction(obj))
                },
                "ChannelOpeningTransaction" => {
                    let obj = ChannelOpeningTransaction::deserialize(value)
                        .map_err(|err|
                            serde::de::Error::custom(format!("Serde JSON Error {}", err))
                        )?;
                    Ok(TransactionEnum::ChannelOpeningTransaction(obj))
                },
                "Deposit" => {
                    let obj = Deposit::deserialize(value)
                        .map_err(|err|
                            serde::de::Error::custom(format!("Serde JSON Error {}", err))
                        )?;
                    Ok(TransactionEnum::Deposit(obj))
                },
                "IncomingPayment" => {
                    let obj = IncomingPayment::deserialize(value)
                        .map_err(|err|
                            serde::de::Error::custom(format!("Serde JSON Error {}", err))
                        )?;
                    Ok(TransactionEnum::IncomingPayment(obj))
                },
                "OutgoingPayment" => {
                    let obj = OutgoingPayment::deserialize(value)
                        .map_err(|err|
                            serde::de::Error::custom(format!("Serde JSON Error {}", err))
                        )?;
                    Ok(TransactionEnum::OutgoingPayment(obj))
                },
                "RoutingTransaction" => {
                    let obj = RoutingTransaction::deserialize(value)
                        .map_err(|err|
                            serde::de::Error::custom(format!("Serde JSON Error {}", err))
                        )?;
                    Ok(TransactionEnum::RoutingTransaction(obj))
                },
                "Withdrawal" => {
                    let obj = Withdrawal::deserialize(value)
                        .map_err(|err|
                            serde::de::Error::custom(format!("Serde JSON Error {}", err))
                        )?;
                    Ok(TransactionEnum::Withdrawal(obj))
                },

                _ => Err(serde::de::Error::custom("unknown typename")),
            }
        } else {
            Err(serde::de::Error::custom("missing __typename field on Transaction"))
        }
    }
}

