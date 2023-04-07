// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Clone, Deserialize, Serialize)]
pub enum WebhookEventType {
    #[serde(rename = "PAYMENT_FINISHED")]
    PaymentFinished,

    #[serde(rename = "NODE_STATUS")]
    NodeStatus,
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
            Self::NodeStatus => write!(f, "NODE_STATUS"),
        }
    }
}
