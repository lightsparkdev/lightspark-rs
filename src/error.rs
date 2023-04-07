// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use std::fmt;

use crate::{crypto::CryptoError, requester::requester::RequesterError};

#[derive(Debug)]
pub enum Error {
    ClientCreationError(String),
    JsonError(serde_json::Error),
    ClientError(RequesterError),
    ConversionError(serde_json::Error),
    CryptoError(CryptoError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ClientCreationError(err) => write!(f, "Client creation error {}", err),
            Self::JsonError(err) => write!(f, "JSON Parser error {}", err),
            Self::ClientError(err) => write!(f, "Client error {}", err),
            Self::ConversionError(err) => write!(f, "Parameter conversion error {}", err),
            Self::CryptoError(err) => write!(f, "Crypto error {}", err),
        }
    }
}

impl std::error::Error for Error {}
