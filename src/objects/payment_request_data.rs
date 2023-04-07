// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use super::invoice_data::InvoiceData;
use crate::objects::bitcoin_network::BitcoinNetwork;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub trait PaymentRequestData {
    fn get_encoded_payment_request(&self) -> String;

    fn get_bitcoin_network(&self) -> BitcoinNetwork;

    fn type_name(&self) -> &'static str;
}

impl<'de> Deserialize<'de> for Box<dyn PaymentRequestData> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        if let Some(typename) = value.get("__typename").and_then(Value::as_str) {
            match typename {
                "InvoiceData" => {
                    let obj = InvoiceData::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(Box::new(obj))
                }

                _ => Err(serde::de::Error::custom("unknown typename")),
            }
        } else {
            Err(serde::de::Error::custom(
                "missing __typename field on PaymentRequestData",
            ))
        }
    }
}
