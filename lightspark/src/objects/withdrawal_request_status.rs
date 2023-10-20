// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum of the potential statuses that a Withdrawal can take.
#[derive(Clone, Deserialize, Serialize)]
pub enum WithdrawalRequestStatus {
    #[serde(rename = "CREATED")]
    Created,

    #[serde(rename = "FAILED")]
    Failed,

    #[serde(rename = "IN_PROGRESS")]
    InProgress,

    #[serde(rename = "SUCCESSFUL")]
    Successful,
}

impl From<WithdrawalRequestStatus> for Value {
    fn from(val: WithdrawalRequestStatus) -> Self {
        Value::from(val.to_string())
    }
}

impl fmt::Display for WithdrawalRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Created => write!(f, "CREATED"),
            Self::Failed => write!(f, "FAILED"),
            Self::InProgress => write!(f, "IN_PROGRESS"),
            Self::Successful => write!(f, "SUCCESSFUL"),
        }
    }
}
