// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum of the potential event types that can be associated with your Lightspark wallets.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum WebhookEventType {
    #[serde(rename = "PAYMENT_FINISHED")]
    PaymentFinished,

    #[serde(rename = "FORCE_CLOSURE")]
    ForceClosure,

    #[serde(rename = "WITHDRAWAL_FINISHED")]
    WithdrawalFinished,

    #[serde(rename = "FUNDS_RECEIVED")]
    FundsReceived,

    #[serde(rename = "NODE_STATUS")]
    NodeStatus,

    #[serde(rename = "UMA_INVITATION_CLAIMED")]
    UmaInvitationClaimed,

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

    #[serde(rename = "LOW_BALANCE")]
    LowBalance,

    #[serde(rename = "HIGH_BALANCE")]
    HighBalance,

    #[serde(rename = "CHANNEL_OPENING_FEES")]
    ChannelOpeningFees,
}

impl Into<Value> for WebhookEventType {
    fn into(self) -> Value {
        Value::from(self.to_string())
    }
}

impl fmt::Display for WebhookEventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PaymentFinished => write!(f, "PAYMENT_FINISHED"),
            Self::ForceClosure => write!(f, "FORCE_CLOSURE"),
            Self::WithdrawalFinished => write!(f, "WITHDRAWAL_FINISHED"),
            Self::FundsReceived => write!(f, "FUNDS_RECEIVED"),
            Self::NodeStatus => write!(f, "NODE_STATUS"),
            Self::UmaInvitationClaimed => write!(f, "UMA_INVITATION_CLAIMED"),
            Self::WalletStatus => write!(f, "WALLET_STATUS"),
            Self::WalletOutgoingPaymentFinished => write!(f, "WALLET_OUTGOING_PAYMENT_FINISHED"),
            Self::WalletIncomingPaymentFinished => write!(f, "WALLET_INCOMING_PAYMENT_FINISHED"),
            Self::WalletWithdrawalFinished => write!(f, "WALLET_WITHDRAWAL_FINISHED"),
            Self::WalletFundsReceived => write!(f, "WALLET_FUNDS_RECEIVED"),
            Self::RemoteSigning => write!(f, "REMOTE_SIGNING"),
            Self::LowBalance => write!(f, "LOW_BALANCE"),
            Self::HighBalance => write!(f, "HIGH_BALANCE"),
            Self::ChannelOpeningFees => write!(f, "CHANNEL_OPENING_FEES"),
        }
    }
}
