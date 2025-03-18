// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum of the potential sub-event types for Remote Signing webook events.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RemoteSigningSubEventType {
    #[serde(rename = "ECDH")]
    Ecdh,

    #[serde(rename = "GET_PER_COMMITMENT_POINT")]
    GetPerCommitmentPoint,

    #[serde(rename = "RELEASE_PER_COMMITMENT_SECRET")]
    ReleasePerCommitmentSecret,

    #[serde(rename = "SIGN_INVOICE")]
    SignInvoice,

    #[serde(rename = "DERIVE_KEY_AND_SIGN")]
    DeriveKeyAndSign,

    #[serde(rename = "RELEASE_PAYMENT_PREIMAGE")]
    ReleasePaymentPreimage,

    #[serde(rename = "REQUEST_INVOICE_PAYMENT_HASH")]
    RequestInvoicePaymentHash,

    #[serde(rename = "REVEAL_COUNTERPARTY_PER_COMMITMENT_SECRET")]
    RevealCounterpartyPerCommitmentSecret,

    #[serde(rename = "VLS_MESSAGE")]
    VlsMessage,
}

impl Into<Value> for RemoteSigningSubEventType {
    fn into(self) -> Value {
        Value::from(self.to_string())
    }
}

impl fmt::Display for RemoteSigningSubEventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Ecdh => write!(f, "ECDH"),
            Self::GetPerCommitmentPoint => write!(f, "GET_PER_COMMITMENT_POINT"),
            Self::ReleasePerCommitmentSecret => write!(f, "RELEASE_PER_COMMITMENT_SECRET"),
            Self::SignInvoice => write!(f, "SIGN_INVOICE"),
            Self::DeriveKeyAndSign => write!(f, "DERIVE_KEY_AND_SIGN"),
            Self::ReleasePaymentPreimage => write!(f, "RELEASE_PAYMENT_PREIMAGE"),
            Self::RequestInvoicePaymentHash => write!(f, "REQUEST_INVOICE_PAYMENT_HASH"),
            Self::RevealCounterpartyPerCommitmentSecret => {
                write!(f, "REVEAL_COUNTERPARTY_PER_COMMITMENT_SECRET")
            }
            Self::VlsMessage => write!(f, "VLS_MESSAGE"),
        }
    }
}
