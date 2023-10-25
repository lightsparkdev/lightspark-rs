use std::fmt::{self, format};

use actix_web::{HttpResponse, Responder};
use chrono::{Duration, Utc};
use serde_json::json;
use uma::protocol::PubKeyResponse;

use crate::config::Config;

pub enum Error {
    SigningKeyParseError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Error::SigningKeyParseError => "Error parsing signing key".to_string(),
        };
        write!(f, "{}", msg)
    }
}

pub trait VASPSending {
    fn handle_client_uma_lookup(&self, receiver: &str) -> String;
    fn handle_client_pay_req(&self, callback_uuid: &str) -> String;
    fn handle_client_payment_confirm(&self, callback_uuid: &str) -> String;
}

pub trait VASPReceiving {
    fn handle_well_known_lnurlp(&self, username: &str) -> String;
    fn handle_lnurl_payreq(&self, uuid: &str) -> String;
    fn handle_uma_payreq(&self, uuid: &str) -> String;
}

#[derive(Debug)]
pub struct VASP {
    pub config: Config,
}

impl VASP {
    pub fn new(config: Config) -> VASP {
        VASP { config }
    }

    pub fn handle_pubkey_request(&self) -> impl Responder {
        let signing_pubkey_bytes = match hex::decode(&self.config.uma_signing_private_key_hex) {
            Ok(bytes) => bytes,
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": "Error parsing signing key",
                }))
            }
        };

        let encryption_pubkey_bytes = match hex::decode(&self.config.uma_encryption_private_key_hex)
        {
            Ok(bytes) => bytes,
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": "Error parsing encryption key",
                }))
            }
        };

        let now = Utc::now();
        let two_weeks_from_now = now + Duration::weeks(2);

        let response = PubKeyResponse {
            signing_pub_key: signing_pubkey_bytes,
            encryption_pub_key: encryption_pubkey_bytes,
            expiration_timestamp: Some(two_weeks_from_now.timestamp()),
        };

        HttpResponse::Ok().json(response)
    }
}

impl VASPSending for VASP {
    fn handle_client_uma_lookup(&self, receiver: &str) -> String {
        format(format_args!("Hello {}!", receiver))
    }

    fn handle_client_pay_req(&self, callback_uuid: &str) -> String {
        format(format_args!("Hello {}!", callback_uuid))
    }

    fn handle_client_payment_confirm(&self, callback_uuid: &str) -> String {
        format(format_args!("Hello {}!", callback_uuid))
    }
}

impl VASPReceiving for VASP {
    fn handle_well_known_lnurlp(&self, username: &str) -> String {
        format(format_args!("Hello {}!", username))
    }

    fn handle_lnurl_payreq(&self, uuid: &str) -> String {
        format(format_args!("Hello {}!", uuid))
    }

    fn handle_uma_payreq(&self, uuid: &str) -> String {
        format(format_args!("Hello {}!", uuid))
    }
}
