// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum of the potential reasons that an attempted routed transaction through a Lightspark node may have failed.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RoutingTransactionFailureReason {
    #[serde(rename = "INCOMING_LINK_FAILURE")]
    IncomingLinkFailure,

    #[serde(rename = "OUTGOING_LINK_FAILURE")]
    OutgoingLinkFailure,

    #[serde(rename = "FORWARDING_FAILURE")]
    ForwardingFailure,
}

impl From<RoutingTransactionFailureReason> for Value {
    fn from(val: RoutingTransactionFailureReason) -> Self {
        Value::from(val.to_string())
    }
}

impl fmt::Display for RoutingTransactionFailureReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IncomingLinkFailure => write!(f, "INCOMING_LINK_FAILURE"),
            Self::OutgoingLinkFailure => write!(f, "OUTGOING_LINK_FAILURE"),
            Self::ForwardingFailure => write!(f, "FORWARDING_FAILURE"),
        }
    }
}
