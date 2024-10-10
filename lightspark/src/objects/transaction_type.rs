
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum of the potential types of transactions that can be associated with your Lightspark Node.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TransactionType {
    /// Transactions initiated from a Lightspark node on Lightning Network.

    #[serde(rename="OUTGOING_PAYMENT")]
    OutgoingPayment,
    /// Transactions received by a Lightspark node on Lightning Network.

    #[serde(rename="INCOMING_PAYMENT")]
    IncomingPayment,
    /// Transactions that forwarded payments through Lightspark nodes on Lightning Network.

    #[serde(rename="ROUTED")]
    Routed,
    /// Transactions on the Bitcoin blockchain to withdraw funds from a Lightspark node to a Bitcoin wallet.

    #[serde(rename="L1_WITHDRAW")]
    L1Withdraw,
    /// Transactions on Bitcoin blockchain to fund a Lightspark node's wallet.

    #[serde(rename="L1_DEPOSIT")]
    L1Deposit,
    /// Transactions on Bitcoin blockchain to open a channel on Lightning Network funded by the local Lightspark node.

    #[serde(rename="CHANNEL_OPEN")]
    ChannelOpen,
    /// Transactions on Bitcoin blockchain to close a channel on Lightning Network where the balances are allocated back to local and remote nodes.

    #[serde(rename="CHANNEL_CLOSE")]
    ChannelClose,
    /// Transactions initiated from a Lightspark node on Lightning Network.

    #[serde(rename="PAYMENT")]
    Payment,
    /// Payment requests from a Lightspark node on Lightning Network

    #[serde(rename="PAYMENT_REQUEST")]
    PaymentRequest,
    /// Transactions that forwarded payments through Lightspark nodes on Lightning Network.

    #[serde(rename="ROUTE")]
    Route,

}

impl Into<Value> for TransactionType {
    fn into(self) -> Value {
        Value::from(self.to_string())
    }
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::OutgoingPayment => write!(f, "OUTGOING_PAYMENT"),
            Self::IncomingPayment => write!(f, "INCOMING_PAYMENT"),
            Self::Routed => write!(f, "ROUTED"),
            Self::L1Withdraw => write!(f, "L1_WITHDRAW"),
            Self::L1Deposit => write!(f, "L1_DEPOSIT"),
            Self::ChannelOpen => write!(f, "CHANNEL_OPEN"),
            Self::ChannelClose => write!(f, "CHANNEL_CLOSE"),
            Self::Payment => write!(f, "PAYMENT"),
            Self::PaymentRequest => write!(f, "PAYMENT_REQUEST"),
            Self::Route => write!(f, "ROUTE"),

        }
    }
}

