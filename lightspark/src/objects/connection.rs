// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use super::account_to_api_tokens_connection::AccountToApiTokensConnection;
use super::account_to_nodes_connection::AccountToNodesConnection;
use super::account_to_payment_requests_connection::AccountToPaymentRequestsConnection;
use super::account_to_transactions_connection::AccountToTransactionsConnection;
use super::account_to_wallets_connection::AccountToWalletsConnection;
use super::account_to_withdrawal_requests_connection::AccountToWithdrawalRequestsConnection;
use super::incoming_payment_to_attempts_connection::IncomingPaymentToAttemptsConnection;
use super::lightspark_node_to_channels_connection::LightsparkNodeToChannelsConnection;
use super::outgoing_payment_attempt_to_hops_connection::OutgoingPaymentAttemptToHopsConnection;
use super::outgoing_payment_to_attempts_connection::OutgoingPaymentToAttemptsConnection;
use super::wallet_to_payment_requests_connection::WalletToPaymentRequestsConnection;
use super::wallet_to_transactions_connection::WalletToTransactionsConnection;
use super::wallet_to_withdrawal_requests_connection::WalletToWithdrawalRequestsConnection;
use crate::objects::page_info::PageInfo;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

pub trait Connection {
    /// The total count of objects in this connection, using the current filters. It is different from the number of objects returned in the current page (in the `entities` field).
    fn get_count(&self) -> i64;

    /// An object that holds pagination information about the objects in this connection.
    fn get_page_info(&self) -> PageInfo;

    fn type_name(&self) -> &'static str;
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize)]
pub enum ConnectionEnum {
    AccountToApiTokensConnection(AccountToApiTokensConnection),
    AccountToNodesConnection(AccountToNodesConnection),
    AccountToPaymentRequestsConnection(AccountToPaymentRequestsConnection),
    AccountToTransactionsConnection(AccountToTransactionsConnection),
    AccountToWalletsConnection(AccountToWalletsConnection),
    AccountToWithdrawalRequestsConnection(AccountToWithdrawalRequestsConnection),
    IncomingPaymentToAttemptsConnection(IncomingPaymentToAttemptsConnection),
    LightsparkNodeToChannelsConnection(LightsparkNodeToChannelsConnection),
    OutgoingPaymentAttemptToHopsConnection(OutgoingPaymentAttemptToHopsConnection),
    OutgoingPaymentToAttemptsConnection(OutgoingPaymentToAttemptsConnection),
    WalletToPaymentRequestsConnection(WalletToPaymentRequestsConnection),
    WalletToTransactionsConnection(WalletToTransactionsConnection),
    WalletToWithdrawalRequestsConnection(WalletToWithdrawalRequestsConnection),
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
                "AccountToNodesConnection" => {
                    let obj = AccountToNodesConnection::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(ConnectionEnum::AccountToNodesConnection(obj))
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
                "AccountToWalletsConnection" => {
                    let obj = AccountToWalletsConnection::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(ConnectionEnum::AccountToWalletsConnection(obj))
                }
                "AccountToWithdrawalRequestsConnection" => {
                    let obj = AccountToWithdrawalRequestsConnection::deserialize(value).map_err(
                        |err| serde::de::Error::custom(format!("Serde JSON Error {}", err)),
                    )?;
                    Ok(ConnectionEnum::AccountToWithdrawalRequestsConnection(obj))
                }
                "IncomingPaymentToAttemptsConnection" => {
                    let obj =
                        IncomingPaymentToAttemptsConnection::deserialize(value).map_err(|err| {
                            serde::de::Error::custom(format!("Serde JSON Error {}", err))
                        })?;
                    Ok(ConnectionEnum::IncomingPaymentToAttemptsConnection(obj))
                }
                "LightsparkNodeToChannelsConnection" => {
                    let obj =
                        LightsparkNodeToChannelsConnection::deserialize(value).map_err(|err| {
                            serde::de::Error::custom(format!("Serde JSON Error {}", err))
                        })?;
                    Ok(ConnectionEnum::LightsparkNodeToChannelsConnection(obj))
                }
                "OutgoingPaymentAttemptToHopsConnection" => {
                    let obj = OutgoingPaymentAttemptToHopsConnection::deserialize(value).map_err(
                        |err| serde::de::Error::custom(format!("Serde JSON Error {}", err)),
                    )?;
                    Ok(ConnectionEnum::OutgoingPaymentAttemptToHopsConnection(obj))
                }
                "OutgoingPaymentToAttemptsConnection" => {
                    let obj =
                        OutgoingPaymentToAttemptsConnection::deserialize(value).map_err(|err| {
                            serde::de::Error::custom(format!("Serde JSON Error {}", err))
                        })?;
                    Ok(ConnectionEnum::OutgoingPaymentToAttemptsConnection(obj))
                }
                "WalletToPaymentRequestsConnection" => {
                    let obj =
                        WalletToPaymentRequestsConnection::deserialize(value).map_err(|err| {
                            serde::de::Error::custom(format!("Serde JSON Error {}", err))
                        })?;
                    Ok(ConnectionEnum::WalletToPaymentRequestsConnection(obj))
                }
                "WalletToTransactionsConnection" => {
                    let obj =
                        WalletToTransactionsConnection::deserialize(value).map_err(|err| {
                            serde::de::Error::custom(format!("Serde JSON Error {}", err))
                        })?;
                    Ok(ConnectionEnum::WalletToTransactionsConnection(obj))
                }
                "WalletToWithdrawalRequestsConnection" => {
                    let obj = WalletToWithdrawalRequestsConnection::deserialize(value).map_err(
                        |err| serde::de::Error::custom(format!("Serde JSON Error {}", err)),
                    )?;
                    Ok(ConnectionEnum::WalletToWithdrawalRequestsConnection(obj))
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
