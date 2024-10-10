
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// Describes the reason for an invitation to not be eligible for incentives.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum IncentivesIneligibilityReason {
    /// This invitation is not eligible for incentives because it has been created outside of the incentives flow.

    #[serde(rename="DISABLED")]
    Disabled,
    /// This invitation is not eligible for incentives because the sender is not eligible.

    #[serde(rename="SENDER_NOT_ELIGIBLE")]
    SenderNotEligible,
    /// This invitation is not eligible for incentives because the receiver is not eligible.

    #[serde(rename="RECEIVER_NOT_ELIGIBLE")]
    ReceiverNotEligible,
    /// This invitation is not eligible for incentives because the sending VASP is not part of the incentives program.

    #[serde(rename="SENDING_VASP_NOT_ELIGIBLE")]
    SendingVaspNotEligible,
    /// This invitation is not eligible for incentives because the receiving VASP is not part of the incentives program.

    #[serde(rename="RECEIVING_VASP_NOT_ELIGIBLE")]
    ReceivingVaspNotEligible,
    /// This invitation is not eligible for incentives because the sender and receiver are in the same region.

    #[serde(rename="NOT_CROSS_BORDER")]
    NotCrossBorder,

}

impl Into<Value> for IncentivesIneligibilityReason {
    fn into(self) -> Value {
        Value::from(self.to_string())
    }
}

impl fmt::Display for IncentivesIneligibilityReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Disabled => write!(f, "DISABLED"),
            Self::SenderNotEligible => write!(f, "SENDER_NOT_ELIGIBLE"),
            Self::ReceiverNotEligible => write!(f, "RECEIVER_NOT_ELIGIBLE"),
            Self::SendingVaspNotEligible => write!(f, "SENDING_VASP_NOT_ELIGIBLE"),
            Self::ReceivingVaspNotEligible => write!(f, "RECEIVING_VASP_NOT_ELIGIBLE"),
            Self::NotCrossBorder => write!(f, "NOT_CROSS_BORDER"),

        }
    }
}

