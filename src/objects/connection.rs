// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use super::account_to_api_tokens_connection::AccountToApiTokensConnection;
use super::account_to_payment_requests_connection::AccountToPaymentRequestsConnection;
use super::account_to_transactions_connection::AccountToTransactionsConnection;
use crate::objects::page_info::PageInfo;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub trait Connection {
    /// The total count of objects in this connection, using the current filters. It is different from the number of objects returned in the current page (in the `entities` field).
    fn get_count(&self) -> i64;

    /// An object that holds pagination information about the objects in this connection.
    fn get_page_info(&self) -> PageInfo;

    fn type_name(&self) -> &'static str;
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum ConnectionEnum {
    AccountToApiTokensConnection(AccountToApiTokensConnection),
    AccountToPaymentRequestsConnection(AccountToPaymentRequestsConnection),
    AccountToTransactionsConnection(AccountToTransactionsConnection),
}

impl<'de> Deserialize<'de> for ConnectionEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        if let Some(typename) = value.get("__typename").and_then(Value::as_str) {
            match typename {
                "AccountToApiTokensConnection" => {
                    let obj = AccountToApiTokensConnection::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(ConnectionEnum::AccountToApiTokensConnection(obj))
                }
                "AccountToPaymentRequestsConnection" => {
                    let obj =
                        AccountToPaymentRequestsConnection::deserialize(value).map_err(|err| {
                            serde::de::Error::custom(format!("Serde JSON Error {}", err))
                        })?;
                    Ok(ConnectionEnum::AccountToPaymentRequestsConnection(obj))
                }
                "AccountToTransactionsConnection" => {
                    let obj =
                        AccountToTransactionsConnection::deserialize(value).map_err(|err| {
                            serde::de::Error::custom(format!("Serde JSON Error {}", err))
                        })?;
                    Ok(ConnectionEnum::AccountToTransactionsConnection(obj))
                }

                _ => Err(serde::de::Error::custom("unknown typename")),
            }
        } else {
            Err(serde::de::Error::custom(
                "missing __typename field on Connection",
            ))
        }
    }
}
