// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum identifying a type of compliance provider.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ComplianceProvider {
    #[serde(rename = "CHAINALYSIS")]
    Chainalysis,
}

impl From<ComplianceProvider> for Value {
    fn from(val: ComplianceProvider) -> Self {
        Value::from(val.to_string())
    }
}

impl fmt::Display for ComplianceProvider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Chainalysis => write!(f, "CHAINALYSIS"),
        }
    }
}
