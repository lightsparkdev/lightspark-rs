// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use super::invoice::Invoice;
use crate::objects::entity::Entity;
use crate::objects::payment_request_data::PaymentRequestData;
use crate::objects::payment_request_status::PaymentRequestStatus;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub trait PaymentRequest: Entity {
    /// The details of the payment request.
    fn get_data(&self) -> &dyn PaymentRequestData;

    /// The status of the payment request.
    fn get_status(&self) -> PaymentRequestStatus;

    fn type_name(&self) -> &'static str;
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum PaymentRequestEnum {
    Invoice(Invoice),
}

impl<'de> Deserialize<'de> for PaymentRequestEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        if let Some(typename) = value.get("__typename").and_then(Value::as_str) {
            match typename {
                "Invoice" => {
                    let obj = Invoice::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(PaymentRequestEnum::Invoice(obj))
                }

                _ => Err(serde::de::Error::custom("unknown typename")),
            }
        } else {
            Err(serde::de::Error::custom(
                "missing __typename field on PaymentRequest",
            ))
        }
    }
}
