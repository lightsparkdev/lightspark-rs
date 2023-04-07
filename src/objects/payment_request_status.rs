// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Clone, Deserialize, Serialize)]
pub enum PaymentRequestStatus {
    #[serde(rename = "OPEN")]
    Open,

    #[serde(rename = "CLOSED")]
    Closed,
}

impl Into<Value> for PaymentRequestStatus {
    fn into(self) -> Value {
        Value::from(self.to_string())
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
