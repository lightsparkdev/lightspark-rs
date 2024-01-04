// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum identifying the payment direction.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LightningPaymentDirection {
    /// A payment that is received by the node.

    #[serde(rename = "INCOMING")]
    Incoming,
    /// A payment that is sent by the node.

    #[serde(rename = "OUTGOING")]
    Outgoing,
}

impl From<LightningPaymentDirection> for Value {
    fn from(val: LightningPaymentDirection) -> Self {
        Value::from(val.to_string())
    }
}

impl fmt::Display for LightningPaymentDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Incoming => write!(f, "INCOMING"),
            Self::Outgoing => write!(f, "OUTGOING"),
        }
    }
}
