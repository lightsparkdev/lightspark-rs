// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
//! Lightspark Remote Signing Library is a library to handle lightspark remote signing webhook
//! events.
//!
//! For most cases, you will want to use the `handler::Handler` to handle the remote signing webhook
//! event you get from lightspark. The handler will process the request, and form a response that
//! you can send back to lightspark through Lightspark SDK.
//!
//! This crates also provides a `signer::LightsparkSigner` that can be used to sign transactions.
use std::fmt;

pub extern crate lightspark;
pub mod handler;
pub mod invoice;
pub mod response;
pub mod signer;
pub mod signing_requests;
pub mod signing_responses;
pub mod validation;

#[derive(Debug)]
pub enum Error {
    WebhookEventNotRemoteSigning,
    WebhookEventDataMissing,
    UnknownSubEventType,
    PublicKeyDecodeError(hex::FromHexError),
    HexEncodingError,
    SignerError(signer::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Error::WebhookEventNotRemoteSigning => {
                "Webhook event is not a remote signing event".to_string()
            }
            Error::WebhookEventDataMissing => "Webhook event data is missing".to_string(),
            Error::UnknownSubEventType => "Unknown sub event type".to_string(),
            Error::PublicKeyDecodeError(e) => format!("Error decoding public key: {}", e),
            Error::HexEncodingError => "Error encoding hex".to_string(),
            Error::SignerError(e) => format!("Error signing: {}", e),
        };
        write!(f, "{}", msg)
    }
}
