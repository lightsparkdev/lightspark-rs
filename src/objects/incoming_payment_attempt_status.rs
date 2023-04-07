// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// Enum that enumerates all the possible status of an incoming payment attempt.
#[derive(Clone, Deserialize, Serialize)]
pub enum IncomingPaymentAttemptStatus {
    #[serde(rename = "ACCEPTED")]
    Accepted,

    #[serde(rename = "SETTLED")]
    Settled,

    #[serde(rename = "CANCELED")]
    Canceled,

    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Into<Value> for IncomingPaymentAttemptStatus {
    fn into(self) -> Value {
        Value::from(self.to_string())
    }
}

impl fmt::Display for IncomingPaymentAttemptStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "ACCEPTED"),
            Self::Settled => write!(f, "SETTLED"),
            Self::Canceled => write!(f, "CANCELED"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}
