// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use super::account::Account;
use super::api_token::ApiToken;
use super::channel::Channel;
use super::channel_closing_transaction::ChannelClosingTransaction;
use super::channel_opening_transaction::ChannelOpeningTransaction;
use super::deposit::Deposit;
use super::graph_node::GraphNode;
use super::hop::Hop;
use super::incoming_payment::IncomingPayment;
use super::incoming_payment_attempt::IncomingPaymentAttempt;
use super::invoice::Invoice;
use super::lightspark_node::LightsparkNode;
use super::outgoing_payment::OutgoingPayment;
use super::outgoing_payment_attempt::OutgoingPaymentAttempt;
use super::routing_transaction::RoutingTransaction;
use super::wallet::Wallet;
use super::withdrawal::Withdrawal;
use super::withdrawal_request::WithdrawalRequest;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub trait Entity {
    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    fn get_id(&self) -> String;

    /// The date and time when the entity was first created.
    fn get_created_at(&self) -> DateTime<Utc>;

    /// The date and time when the entity was last updated.
    fn get_updated_at(&self) -> DateTime<Utc>;

    fn type_name(&self) -> &'static str;
}

impl<'de> Deserialize<'de> for Box<dyn Entity> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        if let Some(typename) = value.get("__typename").and_then(Value::as_str) {
            match typename {
                "Account" => {
                    let obj = Account::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(Box::new(obj))
                }
                "ApiToken" => {
                    let obj = ApiToken::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(Box::new(obj))
                }
                "Channel" => {
                    let obj = Channel::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(Box::new(obj))
                }
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
                "GraphNode" => {
                    let obj = GraphNode::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(Box::new(obj))
                }
                "Hop" => {
                    let obj = Hop::deserialize(value).map_err(|err| {
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
                "IncomingPaymentAttempt" => {
                    let obj = IncomingPaymentAttempt::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(Box::new(obj))
                }
                "Invoice" => {
                    let obj = Invoice::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(Box::new(obj))
                }
                "LightsparkNode" => {
                    let obj = LightsparkNode::deserialize(value).map_err(|err| {
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
                "OutgoingPaymentAttempt" => {
                    let obj = OutgoingPaymentAttempt::deserialize(value).map_err(|err| {
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
                "Wallet" => {
                    let obj = Wallet::deserialize(value).map_err(|err| {
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
                "WithdrawalRequest" => {
                    let obj = WithdrawalRequest::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(Box::new(obj))
                }

                _ => Err(serde::de::Error::custom("unknown typename")),
            }
        } else {
            Err(serde::de::Error::custom(
                "missing __typename field on Entity",
            ))
        }
    }
}
