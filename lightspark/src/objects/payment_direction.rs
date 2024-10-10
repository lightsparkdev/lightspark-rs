// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum indicating the direction of the payment.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PaymentDirection {
    #[serde(rename = "SENT")]
    Sent,

    #[serde(rename = "RECEIVED")]
    Received,
}

impl From<PaymentDirection> for Value {
    fn from(val: PaymentDirection) -> Self {
        Value::from(val.to_string())
    }
}

impl fmt::Display for PaymentDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Sent => write!(f, "SENT"),
            Self::Received => write!(f, "RECEIVED"),
        }
    }
}
