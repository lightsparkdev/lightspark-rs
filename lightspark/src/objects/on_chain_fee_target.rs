// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum OnChainFeeTarget {
    /// Transaction expected to be confirmed within 2 blocks.

    #[serde(rename = "HIGH")]
    High,
    /// Transaction expected to be confirmed within 6 blocks.

    #[serde(rename = "MEDIUM")]
    Medium,
    /// Transaction expected to be confirmed within 18 blocks.

    #[serde(rename = "LOW")]
    Low,
    /// Transaction expected to be confirmed within 50 blocks.

    #[serde(rename = "BACKGROUND")]
    Background,
}

impl Into<Value> for OnChainFeeTarget {
    fn into(self) -> Value {
        Value::from(self.to_string())
    }
}

impl fmt::Display for OnChainFeeTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::High => write!(f, "HIGH"),
            Self::Medium => write!(f, "MEDIUM"),
            Self::Low => write!(f, "LOW"),
            Self::Background => write!(f, "BACKGROUND"),
        }
    }
}
