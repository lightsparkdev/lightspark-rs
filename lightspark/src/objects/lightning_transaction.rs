// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use super::incoming_payment::IncomingPayment;
use super::outgoing_payment::OutgoingPayment;
use super::routing_transaction::RoutingTransaction;
use crate::objects::entity::Entity;
use crate::objects::transaction::Transaction;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub trait LightningTransaction: Transaction + Entity {
    fn type_name(&self) -> &'static str;
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum LightningTransactionEnum {
    IncomingPayment(IncomingPayment),
    OutgoingPayment(OutgoingPayment),
    RoutingTransaction(RoutingTransaction),
}

impl<'de> Deserialize<'de> for LightningTransactionEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        if let Some(typename) = value.get("__typename").and_then(Value::as_str) {
            match typename {
                "IncomingPayment" => {
                    let obj = IncomingPayment::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(LightningTransactionEnum::IncomingPayment(obj))
                }
                "OutgoingPayment" => {
                    let obj = OutgoingPayment::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(LightningTransactionEnum::OutgoingPayment(obj))
                }
                "RoutingTransaction" => {
                    let obj = RoutingTransaction::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(LightningTransactionEnum::RoutingTransaction(obj))
                }

                _ => Err(serde::de::Error::custom("unknown typename")),
            }
        } else {
            Err(serde::de::Error::custom(
                "missing __typename field on LightningTransaction",
            ))
        }
    }
}
