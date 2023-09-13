use std::fmt::format;

use crate::config::Config;

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

    pub fn handle_pubkey_request(&self) -> String {
        format(format_args!("Hello {}!", "pubkey_request"))
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
