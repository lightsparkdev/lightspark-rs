pub mod currency;
pub mod payer_data;
pub mod protocol;
pub mod public_key_cache;

#[cfg(test)]
mod uma_test;

use std::fmt;

use bitcoin::secp256k1::{
    ecdsa::Signature, hashes::sha256, Message, PublicKey, Secp256k1, SecretKey,
};
use rand_core::{OsRng, RngCore};

use self::{
    currency::Currency,
    payer_data::{CompliancePayerData, PayerData, PayerDataOptions},
    protocol::{
        LnurlComplianceResponse, LnurlpRequest, LnurlpResponse, PayReqResponse,
        PayReqResponseCompliance, PayReqResponsePaymentInfo, PayRequest, PubKeyResponse,
    },
};

#[derive(Debug)]
pub enum Error {
    Secp256k1Error(bitcoin::secp256k1::Error),
    EciesSecp256k1Error(ecies::SecpError),
    SignatureFormatError,
    InvalidSignature,
    InvalidResponse,
    ProtocolError(protocol::Error),
    MissingUrlParam(String),
    InvalidUrlPath,
    InvalidHost,
    InvalidData(serde_json::Error),
    CreateInvoiceError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Secp256k1Error(err) => write!(f, "Secp256k1 error {}", err),
            Self::EciesSecp256k1Error(err) => write!(f, "Ecies Secp256k1 error {}", err),
            Self::SignatureFormatError => write!(f, "Signature format error"),
            Self::InvalidSignature => write!(f, "Invalid signature"),
            Self::InvalidResponse => write!(f, "Invalid response"),
            Self::ProtocolError(err) => write!(f, "Protocol error {}", err),
            Self::MissingUrlParam(param) => write!(f, "Missing URL param {}", param),
            Self::InvalidUrlPath => write!(f, "Invalid URL path"),
            Self::InvalidHost => write!(f, "Invalid host"),
            Self::InvalidData(err) => write!(f, "Invalid data {}", err),
            Self::CreateInvoiceError(err) => write!(f, "Create invoice error {}", err),
        }
    }
}

pub fn fetch_public_key_for_vasp<T>(
    vasp_domain: &str,
    mut public_key_cache: T,
) -> Result<PubKeyResponse, Error>
where
    T: public_key_cache::PublicKeyCache,
{
    let publick_key = public_key_cache.fetch_public_key_for_vasp(vasp_domain);
    if let Some(public_key) = publick_key {
        return Ok(public_key.clone());
    }

    let scheme = match vasp_domain.starts_with("localhost:") {
        true => "http",
        false => "https",
    };

    let url = format!("{}//{}/.well-known/lnurlpubkey", scheme, vasp_domain);
    let response = reqwest::blocking::get(url).map_err(|_| Error::InvalidResponse)?;

    if !response.status().is_success() {
        return Err(Error::InvalidResponse);
    }

    let bytes = response.bytes().map_err(|_| Error::InvalidResponse)?;

    let pubkey_response: PubKeyResponse =
        serde_json::from_slice(&bytes).map_err(Error::InvalidData)?;

    public_key_cache.add_public_key_for_vasp(vasp_domain, &pubkey_response);
    Ok(pubkey_response)
}

pub fn generate_nonce() -> String {
    OsRng.next_u64().to_string()
}

fn sign_payload(payload: &[u8], private_key_bytes: &[u8]) -> Result<String, Error> {
    let secp = Secp256k1::new();
    let sk = SecretKey::from_slice(private_key_bytes).map_err(Error::Secp256k1Error)?;
    let msg = Message::from_hashed_data::<sha256::Hash>(payload);
    let signature = secp.sign_ecdsa(&msg, &sk);
    let sig_string = hex::encode(signature.serialize_der());
    Ok(sig_string)
}

fn verify_ecdsa(payload: &[u8], signature: &str, pub_key_bytes: &[u8]) -> Result<(), Error> {
    let sig_bytes = hex::decode(signature).map_err(|_| Error::SignatureFormatError)?;
    let secp = Secp256k1::new();
    let msg = Message::from_hashed_data::<sha256::Hash>(payload);
    let sig = Signature::from_der(&sig_bytes).map_err(Error::Secp256k1Error)?;
    let pk = PublicKey::from_slice(pub_key_bytes).map_err(Error::Secp256k1Error)?;
    secp.verify_ecdsa(&msg, &sig, &pk)
        .map_err(|_| Error::InvalidSignature)
}

pub fn verify_pay_req_signature(
    pay_req: &PayRequest,
    other_vasp_pub_key: &[u8],
) -> Result<(), Error> {
    let payload = pay_req.signable_payload();
    verify_ecdsa(
        &payload,
        &pay_req.payer_data.compliance.signature,
        other_vasp_pub_key,
    )
}

pub fn get_signed_lnurlp_request_url(
    signing_private_key: &[u8],
    receiver_address: &str,
    sender_vasp_domain: &str,
    tr_status: bool,
) -> Result<url::Url, Error> {
    let nonce = generate_nonce();
    let mut unsigned_request = LnurlpRequest {
        receiver_address: receiver_address.to_owned(),
        nonce,
        timestamp: chrono::Utc::now().timestamp(),
        signature: "".to_owned(),
        vasp_domain: sender_vasp_domain.to_owned(),
        tr_status,
    };

    let sig = sign_payload(&unsigned_request.signable_payload(), signing_private_key)?;
    unsigned_request.signature = sig;

    unsigned_request
        .encode_to_url()
        .map_err(Error::ProtocolError)
}

pub fn is_uma_lnurl_query(url: &url::Url) -> bool {
    parse_lnurlp_request(url).is_ok()
}

pub fn parse_lnurlp_request(url: &url::Url) -> Result<LnurlpRequest, Error> {
    let mut query = url.query_pairs();
    let signature = query
        .find(|(key, _)| key == "signature")
        .map(|(_, value)| value)
        .ok_or(Error::MissingUrlParam("signature".to_string()))?;

    let mut query = url.query_pairs();
    let vasp_domain = query
        .find(|(key, _)| key == "vaspDomain")
        .map(|(_, value)| value)
        .ok_or(Error::MissingUrlParam("vsapDomain".to_string()))?;

    let mut query = url.query_pairs();
    let nonce = query
        .find(|(key, _)| key == "nonce")
        .map(|(_, value)| value)
        .ok_or(Error::MissingUrlParam("nonce".to_string()))?;

    let mut query = url.query_pairs();
    let tr_status = query
        .find(|(key, _)| key == "trStatus")
        .map(|(_, value)| value.to_lowercase() == "true")
        .unwrap_or(false);

    let mut query = url.query_pairs();
    let timestamp = query
        .find(|(key, _)| key == "timestamp")
        .map(|(_, value)| value.parse::<i64>())
        .ok_or(Error::MissingUrlParam("timestamp".to_string()))?
        .map_err(|_| Error::MissingUrlParam("timestamp".to_string()))?;

    let path_parts: Vec<&str> = url.path_segments().ok_or(Error::InvalidUrlPath)?.collect();
    if path_parts.len() != 3 || path_parts[0] != ".well-known" || path_parts[1] != "lnurlp" {
        return Err(Error::InvalidUrlPath);
    }

    let receiver_address = format!(
        "{}@{}",
        path_parts[2],
        url.host_str().ok_or(Error::InvalidHost)?
    );

    Ok(LnurlpRequest {
        vasp_domain: vasp_domain.into_owned(),
        signature: signature.into_owned(),
        receiver_address,
        nonce: nonce.into_owned(),
        timestamp,
        tr_status,
    })
}

pub fn verify_uma_lnurl_query_signature(
    query: LnurlpRequest,
    other_vasp_pub_key: &[u8],
) -> Result<(), Error> {
    verify_ecdsa(
        &query.signable_payload(),
        &query.signature,
        other_vasp_pub_key,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn get_lnurlp_response(
    query: &LnurlpRequest,
    private_key_bytes: &[u8],
    requires_travel_rule_info: bool,
    callback: &str,
    encoded_metadata: &str,
    min_sendable_sats: i64,
    max_sendable_sats: i64,
    payer_data_options: &PayerDataOptions,
    currency_options: &[Currency],
    is_receiver_kycd: bool,
) -> Result<LnurlpResponse, Error> {
    let compliance_response = get_signed_compliance_respionse(
        query,
        private_key_bytes,
        requires_travel_rule_info,
        is_receiver_kycd,
    )?;
    Ok(LnurlpResponse {
        tag: "payRequest".to_string(),
        callback: callback.to_string(),
        min_sendable: min_sendable_sats,
        max_sendable: max_sendable_sats,
        encoded_metadata: encoded_metadata.to_string(),
        currencies: currency_options.to_vec(),
        required_payer_data: payer_data_options.clone(),
        compliance: compliance_response,
    })
}

fn get_signed_compliance_respionse(
    query: &LnurlpRequest,
    private_key_bytes: &[u8],
    tr_status: bool,
    is_receiver_kycd: bool,
) -> Result<LnurlComplianceResponse, Error> {
    let timestamp = chrono::Utc::now().timestamp();
    let nonce = generate_nonce();
    let payload_string = format!("{}|{}|{}", query.receiver_address, nonce, timestamp);

    let signature = sign_payload(payload_string.as_bytes(), private_key_bytes)?;

    Ok(LnurlComplianceResponse {
        is_kycd: is_receiver_kycd,
        signature,
        nonce,
        timestamp,
        tr_status,
        receiver_identifier: query.receiver_address.clone(),
    })
}

pub fn verify_uma_lnurlp_response_signature(
    response: LnurlpResponse,
    other_vasp_pub_key: &[u8],
) -> Result<(), Error> {
    let payload = response.signable_payload();
    verify_ecdsa(&payload, &response.compliance.signature, other_vasp_pub_key)
}

pub fn parse_lnurlp_response(bytes: &[u8]) -> Result<LnurlpResponse, Error> {
    serde_json::from_slice(bytes).map_err(Error::InvalidData)
}

#[allow(clippy::too_many_arguments)]
pub fn get_pay_request(
    receiver_encryption_pub_key: &[u8],
    sending_vasp_private_key: &[u8],
    currency_code: &str,
    amount: i64,
    payer_identifier: &str,
    payer_name: Option<&str>,
    payer_email: Option<&str>,
    tr_info: Option<&str>,
    is_payer_kycd: bool,
    payer_uxtos: &[String],
    utxo_callback: &str,
) -> Result<PayRequest, Error> {
    let compliance_data = get_signed_compliance_payer_data(
        receiver_encryption_pub_key,
        sending_vasp_private_key,
        payer_identifier,
        tr_info,
        is_payer_kycd,
        payer_uxtos,
        utxo_callback,
    )?;
    Ok(PayRequest {
        currency_code: currency_code.to_string(),
        amount,
        payer_data: PayerData {
            name: payer_name.map(|s| s.to_string()),
            email: payer_email.map(|s| s.to_string()),
            identifier: payer_identifier.to_string(),
            compliance: compliance_data,
        },
    })
}

fn get_signed_compliance_payer_data(
    receiver_encryption_pub_key: &[u8],
    sending_vasp_private_key: &[u8],
    payer_identifier: &str,
    tr_info: Option<&str>,
    is_payer_kycd: bool,
    payer_uxtos: &[String],
    utxo_callback: &str,
) -> Result<CompliancePayerData, Error> {
    let timestamp = chrono::Utc::now().timestamp();
    let nonce = generate_nonce();

    let encrypted_tr_info = match tr_info {
        Some(tr_info) => Some(encrypt_tr_info(tr_info, receiver_encryption_pub_key)?),
        None => None,
    };
    let payload_string = format!("{}|{}|{}", payer_identifier, nonce, timestamp);
    let signature = sign_payload(payload_string.as_bytes(), sending_vasp_private_key)?;

    Ok(CompliancePayerData {
        utxos: payer_uxtos.to_vec(),
        is_kycd: is_payer_kycd,
        tr_info: encrypted_tr_info,
        signature,
        signature_nonce: nonce,
        signature_timestamp: timestamp,
        utxo_callback: utxo_callback.to_string(),
    })
}

fn encrypt_tr_info(tr_info: &str, receiver_encryption_pub_key: &[u8]) -> Result<String, Error> {
    let cipher_text = ecies::encrypt(receiver_encryption_pub_key, tr_info.as_bytes())
        .map_err(Error::EciesSecp256k1Error)?;
    Ok(hex::encode(cipher_text))
}

pub fn parse_pay_request(bytes: &[u8]) -> Result<PayRequest, Error> {
    serde_json::from_slice(bytes).map_err(Error::InvalidData)
}

pub trait LnurlInvoiceCreator {
    fn create_lnurl_invoice(
        &self,
        node_id: &str,
        master_seed_bytes: &[u8],
        amount_msat: i64,
        metadata: &str,
        expiry_secs: Option<i32>,
    ) -> Result<String, Box<dyn std::error::Error>>;
}

#[allow(clippy::too_many_arguments)]
pub fn get_pay_req_response<T>(
    query: &PayRequest,
    invoice_creator: &T,
    node_id: &str,
    node_master_seed_bytes: &[u8],
    metadata: &str,
    currency_code: &str,
    conversion_rate: i64,
    expiry_secs: i32,
    receiver_channel_utxos: &[String],
    utxo_callback: &str,
) -> Result<PayReqResponse, Error>
where
    T: LnurlInvoiceCreator,
{
    let msats_amount = query.amount * conversion_rate;
    let encoded_payer_data =
        serde_json::to_string(&query.payer_data).map_err(Error::InvalidData)?;
    let encoded_invoice = invoice_creator
        .create_lnurl_invoice(
            node_id,
            node_master_seed_bytes,
            msats_amount,
            &format!("{}{{{}}}", metadata, encoded_payer_data),
            Some(expiry_secs),
        )
        .map_err(|e| Error::CreateInvoiceError(e.to_string()))?;

    Ok(PayReqResponse {
        encoded_invoice,
        routes: [].to_vec(),
        compliance: PayReqResponseCompliance {
            utxos: receiver_channel_utxos.to_vec(),
            utxo_callback: utxo_callback.to_string(),
        },
        payment_info: PayReqResponsePaymentInfo {
            currency_code: currency_code.to_string(),
            multiplier: conversion_rate,
        },
    })
}

pub fn parse_pay_req_response(bytes: &[u8]) -> Result<PayReqResponse, Error> {
    serde_json::from_slice(bytes).map_err(Error::InvalidData)
}
