// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use super::account::Account;
use super::api_token::ApiToken;
use super::channel::Channel;
use super::channel_closing_transaction::ChannelClosingTransaction;
use super::channel_opening_transaction::ChannelOpeningTransaction;
use super::channel_snapshot::ChannelSnapshot;
use super::deposit::Deposit;
use super::graph_node::GraphNode;
use super::hop::Hop;
use super::incoming_payment::IncomingPayment;
use super::incoming_payment_attempt::IncomingPaymentAttempt;
use super::invoice::Invoice;
use super::lightspark_node_with_o_s_k::LightsparkNodeWithOSK;
use super::lightspark_node_with_remote_signing::LightsparkNodeWithRemoteSigning;
use super::offer::Offer;
use super::offer_data::OfferData;
use super::outgoing_payment::OutgoingPayment;
use super::outgoing_payment_attempt::OutgoingPaymentAttempt;
use super::routing_transaction::RoutingTransaction;
use super::signable::Signable;
use super::signable_payload::SignablePayload;
use super::uma_invitation::UmaInvitation;
use super::wallet::Wallet;
use super::withdrawal::Withdrawal;
use super::withdrawal_request::WithdrawalRequest;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
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

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize)]
pub enum EntityEnum {
    Account(Account),
    ApiToken(ApiToken),
    Channel(Channel),
    ChannelClosingTransaction(ChannelClosingTransaction),
    ChannelOpeningTransaction(ChannelOpeningTransaction),
    ChannelSnapshot(ChannelSnapshot),
    Deposit(Deposit),
    GraphNode(GraphNode),
    Hop(Hop),
    IncomingPayment(IncomingPayment),
    IncomingPaymentAttempt(IncomingPaymentAttempt),
    Invoice(Invoice),
    LightsparkNodeWithOSK(LightsparkNodeWithOSK),
    LightsparkNodeWithRemoteSigning(LightsparkNodeWithRemoteSigning),
    Offer(Offer),
    OfferData(OfferData),
    OutgoingPayment(OutgoingPayment),
    OutgoingPaymentAttempt(OutgoingPaymentAttempt),
    RoutingTransaction(RoutingTransaction),
    Signable(Signable),
    SignablePayload(SignablePayload),
    UmaInvitation(UmaInvitation),
    Wallet(Wallet),
    Withdrawal(Withdrawal),
    WithdrawalRequest(WithdrawalRequest),
}

impl<'de> Deserialize<'de> for EntityEnum {
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
                    Ok(EntityEnum::Account(obj))
                }
                "ApiToken" => {
                    let obj = ApiToken::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::ApiToken(obj))
                }
                "Channel" => {
                    let obj = Channel::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::Channel(obj))
                }
                "ChannelClosingTransaction" => {
                    let obj = ChannelClosingTransaction::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::ChannelClosingTransaction(obj))
                }
                "ChannelOpeningTransaction" => {
                    let obj = ChannelOpeningTransaction::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::ChannelOpeningTransaction(obj))
                }
                "ChannelSnapshot" => {
                    let obj = ChannelSnapshot::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::ChannelSnapshot(obj))
                }
                "Deposit" => {
                    let obj = Deposit::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::Deposit(obj))
                }
                "GraphNode" => {
                    let obj = GraphNode::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::GraphNode(obj))
                }
                "Hop" => {
                    let obj = Hop::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::Hop(obj))
                }
                "IncomingPayment" => {
                    let obj = IncomingPayment::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::IncomingPayment(obj))
                }
                "IncomingPaymentAttempt" => {
                    let obj = IncomingPaymentAttempt::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::IncomingPaymentAttempt(obj))
                }
                "Invoice" => {
                    let obj = Invoice::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::Invoice(obj))
                }
                "LightsparkNodeWithOSK" => {
                    let obj = LightsparkNodeWithOSK::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::LightsparkNodeWithOSK(obj))
                }
                "LightsparkNodeWithRemoteSigning" => {
                    let obj =
                        LightsparkNodeWithRemoteSigning::deserialize(value).map_err(|err| {
                            serde::de::Error::custom(format!("Serde JSON Error {}", err))
                        })?;
                    Ok(EntityEnum::LightsparkNodeWithRemoteSigning(obj))
                }
                "Offer" => {
                    let obj = Offer::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::Offer(obj))
                }
                "OfferData" => {
                    let obj = OfferData::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::OfferData(obj))
                }
                "OutgoingPayment" => {
                    let obj = OutgoingPayment::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::OutgoingPayment(obj))
                }
                "OutgoingPaymentAttempt" => {
                    let obj = OutgoingPaymentAttempt::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::OutgoingPaymentAttempt(obj))
                }
                "RoutingTransaction" => {
                    let obj = RoutingTransaction::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::RoutingTransaction(obj))
                }
                "Signable" => {
                    let obj = Signable::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::Signable(obj))
                }
                "SignablePayload" => {
                    let obj = SignablePayload::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::SignablePayload(obj))
                }
                "UmaInvitation" => {
                    let obj = UmaInvitation::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::UmaInvitation(obj))
                }
                "Wallet" => {
                    let obj = Wallet::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::Wallet(obj))
                }
                "Withdrawal" => {
                    let obj = Withdrawal::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::Withdrawal(obj))
                }
                "WithdrawalRequest" => {
                    let obj = WithdrawalRequest::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(EntityEnum::WithdrawalRequest(obj))
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
