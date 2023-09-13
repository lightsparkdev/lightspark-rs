// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum of the potential statuses a transaction associated with your Lightspark Node can take.
#[derive(Clone, Deserialize, Serialize)]
pub enum TransactionStatus {
    /// Transaction succeeded..

    #[serde(rename = "SUCCESS")]
    Success,
    /// Transaction failed.

    #[serde(rename = "FAILED")]
    Failed,
    /// Transaction has been initiated and is currently in-flight.

    #[serde(rename = "PENDING")]
    Pending,
    /// For transaction type PAYMENT_REQUEST only. No payments have been made to a payment request.

    #[serde(rename = "NOT_STARTED")]
    NotStarted,
    /// For transaction type PAYMENT_REQUEST only. A payment request has expired.

    #[serde(rename = "EXPIRED")]
    Expired,
    /// For transaction type PAYMENT_REQUEST only.

    #[serde(rename = "CANCELLED")]
    Cancelled,
}

impl From<TransactionStatus> for Value {
    fn from(val: TransactionStatus) -> Self {
        Value::from(val.to_string())
    }
}

impl fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Success => write!(f, "SUCCESS"),
            Self::Failed => write!(f, "FAILED"),
            Self::Pending => write!(f, "PENDING"),
            Self::NotStarted => write!(f, "NOT_STARTED"),
            Self::Expired => write!(f, "EXPIRED"),
            Self::Cancelled => write!(f, "CANCELLED"),
        }
    }
}
