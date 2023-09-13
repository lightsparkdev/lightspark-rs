use base64::{self, Engine};
use bitcoin::secp256k1::{hashes::sha256, Message, Secp256k1, SecretKey};
use serde_json::json;

use crate::crypto::{self, CryptoError};

pub trait OperationSigningKey: Clone {
    fn new(key_bytes: Vec<u8>) -> Self;
    fn sign_payload(&self, data: &[u8]) -> Result<String, CryptoError>;
}

#[derive(Debug, Clone)]
pub struct RSASigningKey {
    key_bytes: Vec<u8>,
}

impl OperationSigningKey for RSASigningKey {
    fn new(key_bytes: Vec<u8>) -> Self {
        Self { key_bytes }
    }

    fn sign_payload(&self, data: &[u8]) -> Result<String, CryptoError> {
        crypto::sign_payload(&self.key_bytes, data)
    }
}

#[derive(Debug, Clone)]
pub struct Secp256k1SigningKey {
    key_bytes: Vec<u8>,
}

impl OperationSigningKey for Secp256k1SigningKey {
    fn new(key_bytes: Vec<u8>) -> Self {
        Self { key_bytes }
    }

    fn sign_payload(&self, data: &[u8]) -> Result<String, CryptoError> {
        let secp = Secp256k1::new();
        let secret_key =
            SecretKey::from_slice(&self.key_bytes).map_err(CryptoError::Secp256k1Error)?;
        let message = Message::from_hashed_data::<sha256::Hash>(data);
        let sig = secp.sign_ecdsa(&message, &secret_key);

        Ok(json!({
            "v": 1,
            "signature": base64::engine::general_purpose::STANDARD.encode(sig.serialize_der()),
        })
        .to_string())
    }
}
