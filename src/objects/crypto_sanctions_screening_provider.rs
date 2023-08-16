// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum identifying a type of crypto sanctions screening provider.
#[derive(Clone, Deserialize, Serialize)]
pub enum CryptoSanctionsScreeningProvider {
    #[serde(rename = "CHAINALYSIS")]
    Chainalysis,
}

impl From<CryptoSanctionsScreeningProvider> for Value {
    fn from(val: CryptoSanctionsScreeningProvider) -> Self {
        Value::from(val.to_string())
    }
}

impl fmt::Display for CryptoSanctionsScreeningProvider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Chainalysis => write!(f, "CHAINALYSIS"),
        }
    }
}
