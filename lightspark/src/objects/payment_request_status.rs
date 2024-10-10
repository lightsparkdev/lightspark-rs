// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum of the potential states that a payment request on the Lightning Network can take.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PaymentRequestStatus {
    #[serde(rename = "OPEN")]
    Open,

    #[serde(rename = "CLOSED")]
    Closed,
}

impl From<PaymentRequestStatus> for Value {
    fn from(val: PaymentRequestStatus) -> Self {
        Value::from(val.to_string())
    }
}

impl fmt::Display for PaymentRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Open => write!(f, "OPEN"),
            Self::Closed => write!(f, "CLOSED"),
        }
    }
}
