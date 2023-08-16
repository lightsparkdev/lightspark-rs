// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum of the potential reasons why an OutgoingPayment sent from a Lightspark Node may have failed.
#[derive(Clone, Deserialize, Serialize)]
pub enum PaymentFailureReason {
    #[serde(rename = "NONE")]
    None,

    #[serde(rename = "TIMEOUT")]
    Timeout,

    #[serde(rename = "NO_ROUTE")]
    NoRoute,

    #[serde(rename = "ERROR")]
    Error,

    #[serde(rename = "INCORRECT_PAYMENT_DETAILS")]
    IncorrectPaymentDetails,

    #[serde(rename = "INSUFFICIENT_BALANCE")]
    InsufficientBalance,

    #[serde(rename = "INVOICE_ALREADY_PAID")]
    InvoiceAlreadyPaid,

    #[serde(rename = "SELF_PAYMENT")]
    SelfPayment,

    #[serde(rename = "INVOICE_EXPIRED")]
    InvoiceExpired,

    #[serde(rename = "RISK_SCREENING_FAILED")]
    RiskScreeningFailed,
}

impl From<PaymentFailureReason> for Value {
    fn from(val: PaymentFailureReason) -> Self {
        Value::from(val.to_string())
    }
}

impl fmt::Display for PaymentFailureReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => write!(f, "NONE"),
            Self::Timeout => write!(f, "TIMEOUT"),
            Self::NoRoute => write!(f, "NO_ROUTE"),
            Self::Error => write!(f, "ERROR"),
            Self::IncorrectPaymentDetails => write!(f, "INCORRECT_PAYMENT_DETAILS"),
            Self::InsufficientBalance => write!(f, "INSUFFICIENT_BALANCE"),
            Self::InvoiceAlreadyPaid => write!(f, "INVOICE_ALREADY_PAID"),
            Self::SelfPayment => write!(f, "SELF_PAYMENT"),
            Self::InvoiceExpired => write!(f, "INVOICE_EXPIRED"),
            Self::RiskScreeningFailed => write!(f, "RISK_SCREENING_FAILED"),
        }
    }
}
