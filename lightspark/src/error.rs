// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use std::fmt;

use crate::crypto::CryptoError;

#[derive(Debug)]
pub enum Error {
    ReqwestError(String),
    GraphqlError(String),
    InvalidHeaderValue,
    ClientCreationError(String),
    JsonError(serde_json::Error),
    ConversionError(serde_json::Error),
    CryptoError(CryptoError),
    WebhookSignatureError,
    SigningKeyNotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ReqwestError(err) => write!(f, "Reqwest error {}", err),
            Self::GraphqlError(err) => write!(f, "Graphql error {}", err),
            Self::InvalidHeaderValue => write!(f, "Invalid header value"),
            Self::ClientCreationError(err) => write!(f, "Client creation error {}", err),
            Self::JsonError(err) => write!(f, "JSON Parser error {}", err),
            Self::ConversionError(err) => write!(f, "Parameter conversion error {}", err),
            Self::CryptoError(err) => write!(f, "Crypto error {}", err),
            Self::WebhookSignatureError => {
                write!(f, "Webhook message hash does not match signature")
            }
            Self::SigningKeyNotFound => write!(f, "Signing key not found"),
        }
    }
}

impl std::error::Error for Error {}
