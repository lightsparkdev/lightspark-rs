// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use std::fmt;

use aes_gcm::aes::Aes256;
use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit};
use base64::{self, DecodeError, Engine};
use cbc::cipher::block_padding::Pkcs7;
use cbc::cipher::{BlockDecryptMut, KeyIvInit};
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use rsa::sha2::Sha256;
use rsa::signature::{RandomizedSigner, SignatureEncoding};
use rsa::{pkcs1v15::SigningKey, pkcs8::DecodePrivateKey, RsaPrivateKey};
use serde_json::{json, Error, Value};

const KEY_LEN: usize = 32;
const IV_LEN: usize = 12;
const SALT_LEN: usize = 16;
const ITERATIONS: u32 = 500000;

#[derive(Debug)]
pub enum CryptoError {
    UnknownVersionError,
    EncryptionError(aes_gcm::Error),
    DecryptionError,
    JSONParsingError(Error),
    ParsingError(DecodeError),
    Secp256k1Error(bitcoin::secp256k1::Error),
    Bip32Error(bitcoin::bip32::Error),
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnknownVersionError => write!(f, "Unknown version"),
            Self::EncryptionError(err) => write!(f, "Encryption error {}", err),
            Self::DecryptionError => write!(f, "Decryption error"),
            Self::JSONParsingError(err) => write!(f, "JSON Parser error {}", err),
            Self::ParsingError(err) => write!(f, "Graphql Parsing error {}", err),
            Self::Secp256k1Error(err) => write!(f, "Secp256k1 error {}", err),
            Self::Bip32Error(err) => write!(f, "Bip32 error {}", err),
        }
    }
}

impl std::error::Error for CryptoError {}

fn derive_key(password: &[u8], salt: &[u8], iterations: u32) -> Vec<u8> {
    let mut derived = vec![0u8; KEY_LEN];
    pbkdf2_hmac::<Sha256>(password, salt, iterations, &mut derived);
    derived
}

fn derive_key_iv(
    password: &[u8],
    salt: &[u8],
    iterations: u32,
    length: usize,
) -> (Vec<u8>, Vec<u8>) {
    let mut derived = vec![0u8; length];
    pbkdf2_hmac::<Sha256>(password, salt, iterations, &mut derived);
    let (key, iv) = derived.split_at(KEY_LEN);
    (key.to_vec(), iv.to_vec())
}

pub fn encrypt(plaintext: &[u8], password: &str) -> Result<(String, String), CryptoError> {
    let mut salt = [0u8; SALT_LEN];
    rand::thread_rng().fill_bytes(&mut salt);
    let (key, iv) = derive_key_iv(password.as_bytes(), &salt, ITERATIONS, KEY_LEN + IV_LEN);
    let key: &Key<Aes256Gcm> = key.as_slice().into();

    let cipher = Aes256Gcm::new(key);
    let ciphertext = cipher
        .encrypt(iv.as_slice().into(), plaintext)
        .map_err(CryptoError::EncryptionError)?;
    let header = json!({"v": 4, "i": ITERATIONS});
    Ok((
        header.to_string(),
        base64::engine::general_purpose::STANDARD.encode([&salt[..], &ciphertext[..]].concat()),
    ))
}

type Aes256CbcDec = cbc::Decryptor<Aes256>;

fn decrypt_cbc(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let mut buf = [0u8; 48];
    let pt = Aes256CbcDec::new(key.into(), iv.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(ciphertext, &mut buf)
        .map_err(|_| CryptoError::DecryptionError)?;
    Ok(pt.to_vec())
}

fn decrypt_gcm(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let key: &Key<Aes256Gcm> = key.into();
    let cipher = Aes256Gcm::new(key);

    let plaintext = cipher
        .decrypt(iv.into(), ciphertext)
        .map_err(|_| CryptoError::DecryptionError)?;
    Ok(plaintext)
}

pub fn decrypt_private_key(
    cipher_version: &str,
    encrypted_value: &str,
    password: &str,
) -> Result<Vec<u8>, CryptoError> {
    let mut decoded = base64::engine::general_purpose::STANDARD
        .decode(encrypted_value)
        .map_err(CryptoError::ParsingError)?;

    let mut header: Value;
    if cipher_version == "AES_256_CBC_PBKDF2_5000_SHA256" {
        header = json!({"v": 0, "i": 5000});
        decoded = decoded[8..].to_vec();
    } else {
        header = serde_json::from_str(cipher_version).map_err(CryptoError::JSONParsingError)?;
        if header.get("lsv").unwrap_or(&Value::from(0)) == 2 {
            header["v"] = Value::from(3);
        }
        let v = header["v"].as_i64().unwrap_or(-1);
        if !(0..=4).contains(&v) {
            return Err(CryptoError::UnknownVersionError);
        }
        if v == 3 {
            let i = header["i"].as_i64().unwrap_or(0);
            let salt_len = decoded.len() - 8;
            let salt = &decoded[salt_len..];
            let nonce = &decoded[0..12];
            let ciphertext = &decoded[12..salt_len];
            let key = derive_key(password.as_bytes(), salt, i as u32);
            return decrypt_gcm(ciphertext, &key, nonce);
        }
    }
    let v = header["v"].as_i64().unwrap_or(-1);
    let i = header["i"].as_i64().unwrap_or(0);
    let salt_len = if v < 4 { 8 } else { SALT_LEN };
    let iv_len = if v < 4 { 16 } else { IV_LEN };
    let salt = &decoded[..salt_len];
    let ciphertext = &decoded[salt_len..];
    let (key, iv) = derive_key_iv(password.as_bytes(), salt, i as u32, KEY_LEN + iv_len);
    if v < 2 {
        decrypt_cbc(ciphertext, &key, &iv)
    } else {
        decrypt_gcm(ciphertext, &key, &iv)
    }
}

pub fn sign_payload(payload: &[u8], signing_key: &[u8]) -> Result<String, CryptoError> {
    let signing_key = if signing_key[0] != 48 {
        base64::engine::general_purpose::STANDARD
            .decode(signing_key)
            .map_err(CryptoError::ParsingError)?
    } else {
        signing_key.to_vec()
    };

    let key = RsaPrivateKey::from_pkcs8_der(&signing_key).expect("Fail to generate key");
    let signing_key = SigningKey::<Sha256>::new(key);
    let mut rng = rand::thread_rng();

    let signature = signing_key.sign_with_rng(&mut rng, payload);

    Ok(json!({
        "v": 1,
        "signature": base64::engine::general_purpose::STANDARD.encode(signature.to_bytes()),
    })
    .to_string())
}
