// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum for potential invoice types.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum InvoiceType {
    /// A standard Bolt 11 invoice.

    #[serde(rename = "STANDARD")]
    Standard,
    /// An AMP (Atomic Multi-path Payment) invoice.

    #[serde(rename = "AMP")]
    Amp,
}

impl From<InvoiceType> for Value {
    fn from(val: InvoiceType) -> Self {
        Value::from(val.to_string())
    }
}

impl fmt::Display for InvoiceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Standard => write!(f, "STANDARD"),
            Self::Amp => write!(f, "AMP"),
        }
    }
}
