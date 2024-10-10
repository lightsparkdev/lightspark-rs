
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum of all potential statuses of a payment attempt made from a Lightspark Node.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum OutgoingPaymentAttemptStatus {

    #[serde(rename="IN_FLIGHT")]
    InFlight,

    #[serde(rename="SUCCEEDED")]
    Succeeded,

    #[serde(rename="FAILED")]
    Failed,

}

impl Into<Value> for OutgoingPaymentAttemptStatus {
    fn into(self) -> Value {
        Value::from(self.to_string())
    }
}

impl fmt::Display for OutgoingPaymentAttemptStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InFlight => write!(f, "IN_FLIGHT"),
            Self::Succeeded => write!(f, "SUCCEEDED"),
            Self::Failed => write!(f, "FAILED"),

        }
    }
}

