
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// Describes the status of the incentives for this invitation.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum IncentivesStatus {
    /// The invitation is eligible for incentives in its current state. When it is claimed, we will reassess.

    #[serde(rename="PENDING")]
    Pending,
    /// The incentives have been validated.

    #[serde(rename="VALIDATED")]
    Validated,
    /// This invitation is not eligible for incentives. A more detailed reason can be found in the `incentives_ineligibility_reason` field.

    #[serde(rename="INELIGIBLE")]
    Ineligible,

}

impl Into<Value> for IncentivesStatus {
    fn into(self) -> Value {
        Value::from(self.to_string())
    }
}

impl fmt::Display for IncentivesStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "PENDING"),
            Self::Validated => write!(f, "VALIDATED"),
            Self::Ineligible => write!(f, "INELIGIBLE"),

        }
    }
}

