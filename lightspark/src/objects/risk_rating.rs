// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum of the potential risk ratings related to a transaction made over the Lightning Network. These risk ratings are returned from the CryptoSanctionScreeningProvider.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RiskRating {
    #[serde(rename = "HIGH_RISK")]
    HighRisk,

    #[serde(rename = "LOW_RISK")]
    LowRisk,

    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl From<RiskRating> for Value {
    fn from(val: RiskRating) -> Self {
        Value::from(val.to_string())
    }
}

impl fmt::Display for RiskRating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::HighRisk => write!(f, "HIGH_RISK"),
            Self::LowRisk => write!(f, "LOW_RISK"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}
