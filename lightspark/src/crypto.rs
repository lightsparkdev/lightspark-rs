// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use std::fmt;

use base64::{self, DecodeError, Engine};
use openssl::error::ErrorStack;
use openssl::hash::MessageDigest;
use openssl::pkcs5::pbkdf2_hmac;
use openssl::pkey::PKey;
use openssl::rsa::Padding;
use openssl::sign::Signer;
use openssl::symm::{Cipher, Crypter, Mode};
use rand::RngCore;
use serde_json::{json, Error, Value};

const KEY_LEN: usize = 32;
const IV_LEN: usize = 12;
const SALT_LEN: usize = 16;
const ITERATIONS: usize = 500000;

#[derive(Debug)]
pub enum CryptoError {
    DeriveKeyError(ErrorStack),
    UnknownVersionError,
    DeriveIVError(ErrorStack),
    EncryptionError(ErrorStack),
    DecryptionError(ErrorStack),
    JSONParsingError(Error),
    ParsingError(DecodeError),
    SigningError(ErrorStack),
    Secp256k1Error(bitcoin::secp256k1::Error),
    Bip32Error(bitcoin::bip32::Error),
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DeriveKeyError(err) => write!(f, "Derive key error {}", err),
            Self::UnknownVersionError => write!(f, "Unknown version"),
            Self::DeriveIVError(err) => write!(f, "Derive IV error {}", err),
            Self::EncryptionError(err) => write!(f, "Encryption error {}", err),
            Self::DecryptionError(err) => write!(f, "Decryption error {}", err),
            Self::JSONParsingError(err) => write!(f, "JSON Parser error {}", err),
            Self::ParsingError(err) => write!(f, "Graphql Parsing error {}", err),
            Self::SigningError(err) => write!(f, "Signing error {}", err),
            Self::Secp256k1Error(err) => write!(f, "Secp256k1 error {}", err),
            Self::Bip32Error(err) => write!(f, "Bip32 error {}", err),
        }
    }
}

impl std::error::Error for CryptoError {}

fn derive_key(password: &[u8], salt: &[u8], iterations: usize) -> Result<Vec<u8>, CryptoError> {
    let mut derived = vec![0u8; KEY_LEN];
    pbkdf2_hmac(
        password,
        salt,
        iterations,
        MessageDigest::sha256(),
        &mut derived,
    )
    .map_err(CryptoError::DeriveKeyError)?;
    Ok(derived)
}

fn derive_key_iv(
    password: &[u8],
    salt: &[u8],
    iterations: usize,
    length: usize,
) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
    let mut derived = vec![0u8; length];
    pbkdf2_hmac(
        password,
        salt,
        iterations,
        MessageDigest::sha256(),
        &mut derived,
    )
    .map_err(CryptoError::DeriveIVError)?;
    let (key, iv) = derived.split_at(KEY_LEN);
    Ok((key.to_vec(), iv.to_vec()))
}

pub fn encrypt(plaintext: &[u8], password: &str) -> Result<(String, String), CryptoError> {
    let mut salt = [0u8; SALT_LEN];
    rand::thread_rng().fill_bytes(&mut salt);
    let (key, iv) = derive_key_iv(password.as_bytes(), &salt, ITERATIONS, KEY_LEN + IV_LEN)?;
    let cipher = Cipher::aes_256_gcm();
    let mut encryptor = Crypter::new(cipher, Mode::Encrypt, &key, Some(&iv))
        .map_err(CryptoError::EncryptionError)?;
    let mut ciphertext = vec![0; plaintext.len() + cipher.block_size()];
    let mut count = encryptor
        .update(plaintext, &mut ciphertext)
        .map_err(CryptoError::EncryptionError)?;
    count += encryptor
        .finalize(&mut ciphertext[count..])
        .map_err(CryptoError::EncryptionError)?;
    ciphertext.truncate(count);
    let header = json!({"v": 4, "i": ITERATIONS});
    Ok((
        header.to_string(),
        base64::engine::general_purpose::STANDARD.encode([&salt[..], &ciphertext[..]].concat()),
    ))
}

fn decrypt_cbc(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let cipher = Cipher::aes_256_cbc();
    let mut decryptor =
        Crypter::new(cipher, Mode::Decrypt, key, Some(iv)).map_err(CryptoError::DecryptionError)?;
    let block_size = cipher.block_size();
    let mut plaintext = vec![0; ciphertext.len() + block_size];
    let count = decryptor
        .update(ciphertext, &mut plaintext)
        .map_err(CryptoError::DecryptionError)?;
    let _ = decryptor.finalize(&mut plaintext);
    plaintext.truncate(count);
    let pad_len = plaintext[plaintext.len() - 1] as usize;
    plaintext.truncate(plaintext.len() - pad_len);
    Ok(plaintext)
}

fn decrypt_gcm(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let cipher = Cipher::aes_256_gcm();
    let mut decryptor =
        Crypter::new(cipher, Mode::Decrypt, key, Some(iv)).map_err(CryptoError::DecryptionError)?;
    let mut plaintext = vec![0; ciphertext.len()];
    let count = decryptor
        .update(ciphertext, &mut plaintext)
        .map_err(CryptoError::DecryptionError)?;
    let _ = decryptor.finalize(&mut plaintext);
    plaintext.truncate(count);
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
            let key = derive_key(password.as_bytes(), salt, i as usize)?;
            return decrypt_gcm(ciphertext, &key, nonce);
        }
    }
    let v = header["v"].as_i64().unwrap_or(-1);
    let i = header["i"].as_i64().unwrap_or(0);
    let salt_len = if v < 4 { 8 } else { SALT_LEN };
    let iv_len = if v < 4 { 16 } else { IV_LEN };
    let salt = &decoded[..salt_len];
    let ciphertext = &decoded[salt_len..];
    let (key, iv) = derive_key_iv(password.as_bytes(), salt, i as usize, KEY_LEN + iv_len)?;
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
    let key = PKey::private_key_from_der(&signing_key).map_err(CryptoError::SigningError)?;
    let mut signer =
        Signer::new(MessageDigest::sha256(), &key).map_err(CryptoError::SigningError)?;
    signer
        .set_rsa_padding(Padding::PKCS1_PSS)
        .map_err(CryptoError::SigningError)?;
    signer.update(payload).map_err(CryptoError::SigningError)?;
    let signature = signer.sign_to_vec().map_err(CryptoError::SigningError)?;

    Ok(json!({
        "v": 1,
        "signature": base64::engine::general_purpose::STANDARD.encode(signature),
    })
    .to_string())
}
