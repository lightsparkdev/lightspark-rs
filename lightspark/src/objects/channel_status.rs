// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum representing the status of a channel on the Lightning Network.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ChannelStatus {
    /// The channel is online and ready to send and receive funds.

    #[serde(rename = "OK")]
    Ok,
    /// The channel has been created, but the Bitcoin transaction that initiates it still needs to be confirmed on the Bitcoin blockchain.

    #[serde(rename = "PENDING")]
    Pending,
    /// The channel is not available, likely because the peer is not online.

    #[serde(rename = "OFFLINE")]
    Offline,
    /// The channel is behaving properly, but its remote balance is much higher than its local balance so it is not balanced properly for sending funds out.

    #[serde(rename = "UNBALANCED_FOR_SEND")]
    UnbalancedForSend,
    /// The channel is behaving properly, but its remote balance is much lower than its local balance so it is not balanced properly for receiving funds.

    #[serde(rename = "UNBALANCED_FOR_RECEIVE")]
    UnbalancedForReceive,
    /// The channel has been closed. Information about the channel is still available for historical purposes but the channel cannot be used anymore.

    #[serde(rename = "CLOSED")]
    Closed,
    /// Something unexpected happened and we cannot determine the status of this channel. Please try again later or contact the support.

    #[serde(rename = "ERROR")]
    Error,
}

impl Into<Value> for ChannelStatus {
    fn into(self) -> Value {
        Value::from(self.to_string())
    }
}

impl fmt::Display for ChannelStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Ok => write!(f, "OK"),
            Self::Pending => write!(f, "PENDING"),
            Self::Offline => write!(f, "OFFLINE"),
            Self::UnbalancedForSend => write!(f, "UNBALANCED_FOR_SEND"),
            Self::UnbalancedForReceive => write!(f, "UNBALANCED_FOR_RECEIVE"),
            Self::Closed => write!(f, "CLOSED"),
            Self::Error => write!(f, "ERROR"),
        }
    }
}
