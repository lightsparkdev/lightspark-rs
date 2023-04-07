// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Clone, Deserialize, Serialize)]
pub enum WithdrawalRequestStatus {
    #[serde(rename = "FAILED")]
    Failed,

    #[serde(rename = "IN_PROGRESS")]
    InProgress,

    #[serde(rename = "SUCCESSFUL")]
    Successful,
}

impl Into<Value> for WithdrawalRequestStatus {
    fn into(self) -> Value {
        Value::from(self.to_string())
    }
}

impl fmt::Display for WithdrawalRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Failed => write!(f, "FAILED"),
            Self::InProgress => write!(f, "IN_PROGRESS"),
            Self::Successful => write!(f, "SUCCESSFUL"),
        }
    }
}
