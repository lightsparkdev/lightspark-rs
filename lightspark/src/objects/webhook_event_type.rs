// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum of the potential event types that can be associated with your Lightspark wallets.
#[derive(Clone, Deserialize, Serialize)]
pub enum WebhookEventType {
    #[serde(rename = "PAYMENT_FINISHED")]
    PaymentFinished,

    #[serde(rename = "NODE_STATUS")]
    NodeStatus,

    #[serde(rename = "WALLET_STATUS")]
    WalletStatus,

    #[serde(rename = "WALLET_OUTGOING_PAYMENT_FINISHED")]
    WalletOutgoingPaymentFinished,

    #[serde(rename = "WALLET_INCOMING_PAYMENT_FINISHED")]
    WalletIncomingPaymentFinished,

    #[serde(rename = "WALLET_WITHDRAWAL_FINISHED")]
    WalletWithdrawalFinished,

    #[serde(rename = "WALLET_FUNDS_RECEIVED")]
    WalletFundsReceived,

    #[serde(rename = "REMOTE_SIGNING")]
    RemoteSigning,
}

impl From<WebhookEventType> for Value {
    fn from(val: WebhookEventType) -> Self {
        Value::from(val.to_string())
    }
}

impl fmt::Display for WebhookEventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PaymentFinished => write!(f, "PAYMENT_FINISHED"),
            Self::NodeStatus => write!(f, "NODE_STATUS"),
            Self::WalletStatus => write!(f, "WALLET_STATUS"),
            Self::WalletOutgoingPaymentFinished => write!(f, "WALLET_OUTGOING_PAYMENT_FINISHED"),
            Self::WalletIncomingPaymentFinished => write!(f, "WALLET_INCOMING_PAYMENT_FINISHED"),
            Self::WalletWithdrawalFinished => write!(f, "WALLET_WITHDRAWAL_FINISHED"),
            Self::WalletFundsReceived => write!(f, "WALLET_FUNDS_RECEIVED"),
            Self::RemoteSigning => write!(f, "REMOTE_SIGNING"),
        }
    }
}
