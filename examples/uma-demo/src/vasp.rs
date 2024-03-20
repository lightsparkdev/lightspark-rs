use std::fmt::{self, format};

use actix_web::{HttpRequest, HttpResponse, Responder};
use chrono::{Duration, Utc};
use lightspark::{
    client::LightsparkClient,
    key::Secp256k1SigningKey,
    objects::{
        lightspark_node::LightsparkNode,
        lightspark_node_with_remote_signing::LightsparkNodeWithRemoteSigning,
        outgoing_payment::OutgoingPayment, transaction_status::TransactionStatus,
    },
    utils::value_millisatoshi,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uma::{
    payer_data::PayerDataOptions,
    protocol::{PubKeyResponse, UtxoWithAmount},
    public_key_cache::PublicKeyCache,
    uma::{
        fetch_public_key_for_vasp, get_pay_request, get_signed_lnurlp_request_url,
        is_uma_lnurl_query, parse_lnurlp_response, verify_uma_lnurlp_response_signature,
    },
};
use url::Url;

use crate::{config::Config, vasp_request_cache::Vasp1PayReqCache};

pub enum Error {
    SigningKeyParseError,
    PaymentTimeOut,
    LightsparkError(lightspark::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Error::SigningKeyParseError => "Error parsing signing key".to_string(),
            Error::PaymentTimeOut => "Payment timed out".to_string(),
            Error::LightsparkError(e) => format!("Lightspark error: {}", e),
        };
        write!(f, "{}", msg)
    }
}

pub struct SendingVASP<T: PublicKeyCache> {
    pub config: Config,
    pub pubkey_cache: T,
    pub request_cache: Vasp1PayReqCache,
    pub client: LightsparkClient<Secp256k1SigningKey>,
}

impl<T: PublicKeyCache> SendingVASP<T> {
    pub fn new(
        config: Config,
        pubkey_cache: T,
        request_cache: Vasp1PayReqCache,
        client: LightsparkClient<Secp256k1SigningKey>,
    ) -> SendingVASP<T> {
        SendingVASP {
            config,
            pubkey_cache,
            request_cache,
            client,
        }
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
        let two_weeks_from_now = now + Duration::try_weeks(2).unwrap();

        let response = PubKeyResponse {
            signing_pub_key: signing_pubkey_bytes,
            encryption_pub_key: encryption_pubkey_bytes,
            expiration_timestamp: Some(two_weeks_from_now.timestamp()),
        };

        HttpResponse::Ok().json(response)
    }

    pub fn handle_client_uma_lookup(&mut self, receiver: &str) -> impl Responder {
        let address_parts = receiver.split_terminator('@').collect::<Vec<&str>>();
        if address_parts.len() != 2 {
            return HttpResponse::BadRequest().json(json!({
                "status": "ERROR",
                "reason": "Invalid receiver address",
            }));
        }

        let receiver_id = address_parts[0];
        let receiver_vasp = address_parts[1];
        let signing_key = match hex::decode(&self.config.uma_signing_private_key_hex) {
            Ok(bytes) => bytes,
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": "Error parsing signing key",
                }))
            }
        };

        let lnurlp_request = match get_signed_lnurlp_request_url(
            &signing_key,
            receiver,
            "localhost:8080",
            true,
            None,
        ) {
            Ok(url) => url,
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": "Error generating lnurlp request",
                }))
            }
        };

        let response = match reqwest::blocking::get(lnurlp_request) {
            Ok(response) => response,
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": "Error fetching lnurlp request",
                }))
            }
        };

        if response.status() == reqwest::StatusCode::PRECONDITION_FAILED {
            todo!("handle unsupported version")
        }

        if response.status() != reqwest::StatusCode::OK {
            return HttpResponse::InternalServerError().json(json!({
                "status": "ERROR",
                "reason": format!("Failed response from receiver: {}", response.status()),
            }));
        }

        let response_body = match response.text() {
            Ok(body) => body,
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": "Error reading response body",
                }))
            }
        };

        let lnurlp_response = match parse_lnurlp_response(response_body.as_bytes()) {
            Ok(response) => response,
            Err(e) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": format!("Failed to parse lnurlp response from receiver: {}", e),
                }))
            }
        };

        let receiving_vasp_pubkey =
            match fetch_public_key_for_vasp(receiver_vasp, &mut self.pubkey_cache) {
                Ok(pubkey) => pubkey,
                Err(_) => {
                    return HttpResponse::InternalServerError().json(json!({
                        "status": "ERROR",
                        "reason": "Failed to fetch public key for receiving VASP",
                    }))
                }
            };

        let receving_signing_pubkey = receiving_vasp_pubkey.signing_pub_key;

        let result =
            verify_uma_lnurlp_response_signature(&lnurlp_response, &receving_signing_pubkey);

        if result.is_err() {
            return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": "Failed to verify lnurlp response signature from receiver",
            }));
        }

        let callback_uuid = self.request_cache.save_lnurlp_response_data(
            &lnurlp_response,
            receiver_id,
            receiver_vasp,
        );

        HttpResponse::Ok().json(json!({
            "currencies":        lnurlp_response.currencies.clone(),
            "minSendSats":       lnurlp_response.min_sendable.clone(),
            "maxSendSats":       lnurlp_response.max_sendable.clone(),
            "callbackUuid":      callback_uuid,
            "receiverKYCStatus": lnurlp_response.compliance.kyc_status.clone(), // You might not actually send this to a client in practice.
        }))
    }

    pub async fn handle_client_pay_req(
        &mut self,
        callback_uuid: &str,
        amount: i64,
        currency_code: &str,
    ) -> impl Responder {
        let initial_request_data = match self.request_cache.get_lnurlp_response_data(callback_uuid)
        {
            Some(data) => data,
            None => {
                return HttpResponse::BadRequest().json(json!({
                    "status": "ERROR",
                    "reason": "Invalid or missing callback UUID",
                }))
            }
        };

        if amount <= 0 {
            return HttpResponse::BadRequest().json(json!({
                "status": "ERROR",
                "reason": "Invalid amount",
            }));
        }

        let mut currency_supported = false;
        for currency in initial_request_data.lnurl_response.currencies.iter() {
            if currency.code == currency_code {
                currency_supported = true;
                break;
            }
        }

        if !currency_supported {
            return HttpResponse::BadRequest().json(json!({
                "status": "ERROR",
                "reason": "Unsupported currency",
            }));
        }

        let uma_signing_private_key = match hex::decode(&self.config.uma_signing_private_key_hex) {
            Ok(bytes) => bytes,
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": "Error parsing signing key",
                }))
            }
        };

        let vasp2_pubkey = match fetch_public_key_for_vasp(
            &initial_request_data.vasp2_domain,
            &mut self.pubkey_cache,
        ) {
            Ok(pubkey) => pubkey,
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": "Failed to fetch public key for receiving VASP",
                }))
            }
        };

        let payer_info = get_payer_info(&initial_request_data.lnurl_response.required_payer_data);
        let tr_info = "Here is some fake travel rul info. It's up to you to implement this.";
        let node = match self
            .client
            .get_entity::<LightsparkNodeWithRemoteSigning>(&self.config.node_id)
            .await
        {
            Ok(node) => node,
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": "Failed to fetch node",
                }))
            }
        };

        let sender_utxos = node.get_uma_prescreening_utxos();

        let vasp2_encryption_pubkey = vasp2_pubkey.encryption_pub_key;

        // This is the node pub key of the sender's node. In practice, you'd want to get this from the sender's node.
        let sender_node_pubkey = "048e45c1f79463468f5824931ac5e34295426a0f954126903e2e3be0aa649e798b708944ba27c0be0a337362bde7f8e474ea8182b2ede5b8980f30e00af5b5df2e";

        // In practice, you'd probably use some real transaction ID here.
        let _tx_id = "1234";

        let pay_req = match get_pay_request(
            &vasp2_encryption_pubkey,
            &uma_signing_private_key,
            currency_code,
            amount,
            &payer_info.identifier,
            payer_info.name.as_deref(),
            payer_info.email.as_deref(),
            Some(tr_info),
            None,
            uma::kyc_status::KycStatus::KycStatusVerified,
            &sender_utxos,
            Some(sender_node_pubkey),
            "", // TODO: get the utxo callback url
        ) {
            Ok(pay_req) => pay_req,
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": "Error generating pay request",
                }))
            }
        };

        let pay_req_bytes = match serde_json::to_vec(&pay_req) {
            Ok(bytes) => bytes,
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": "Error serializing pay request",
                }))
            }
        };

        let pay_req_result = match reqwest::blocking::Client::new()
            .post(initial_request_data.lnurl_response.callback.clone())
            .header("Content-Type", "application/json")
            .body(pay_req_bytes)
            .send()
        {
            Ok(result) => result,
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": "Error sending pay request",
                }))
            }
        };

        if pay_req_result.status() != reqwest::StatusCode::OK {
            return HttpResponse::InternalServerError().json(json!({
                "status": "ERROR",
                "reason": format!("Failed response from receiver: {}", pay_req_result.status()),
            }));
        }

        let pay_req_response = match pay_req_result.text() {
            Ok(body) => body,
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": "Error reading response body",
                }))
            }
        };

        let pay_req_response = match serde_json::from_str::<uma::protocol::PayReqResponse>(
            pay_req_response.as_str(),
        ) {
            Ok(response) => response,
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": "Error parsing pay request response",
                }))
            }
        };

        // TODO: Pre-screen the UTXOs from payreqResponse.Compliance.Utxos
        println!("Received invoice: {}", pay_req_response.encoded_invoice);
        let invoice = match self
            .client
            .get_decoded_payment_request(&pay_req_response.encoded_invoice)
            .await
        {
            Ok(invoice) => invoice,
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": "Error decoding invoice",
                }))
            }
        };

        self.request_cache.save_pay_req_data(
            callback_uuid,
            &pay_req_response.encoded_invoice,
            &pay_req_response.compliance.utxo_callback,
            &invoice,
        );

        HttpResponse::Ok().json(json!({
        "encodedInvoice": pay_req_response.encoded_invoice,
        "callbackUuid":   callback_uuid,
        "amount":         value_millisatoshi(&invoice.amount).unwrap(),
        "conversionRate": pay_req_response.payment_info.multiplier,
        "currencyCode":   pay_req_response.payment_info.currency_code,
        "expiresAt":      invoice.expires_at.timestamp(),
        }))
    }

    pub async fn handle_client_payment_confirm(&mut self, callback_uuid: &str) -> impl Responder {
        let pay_req_data = match self.request_cache.get_pay_req_data(callback_uuid) {
            Some(data) => data,
            None => {
                return HttpResponse::BadRequest().json(json!({
                        "status": "ERROR",
                        "reason": "Invalid or missing callback UUID",
                }))
            }
        };

        if pay_req_data.invoice_data.amount.original_value == 0 {
            return HttpResponse::BadRequest().json(json!({
                    "status": "ERROR",
                    "reason": "cannot pay zero-amount invoices via UMA",
            }));
        }

        let seed_bytes = match hex::decode(&self.config.node_master_seed_hex) {
            Ok(bytes) => bytes,
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": "Error parsing node master seed",
                }))
            }
        };

        match self.client.provide_master_seed(
            &self.config.node_id,
            seed_bytes.to_vec(),
            lightspark::objects::bitcoin_network::BitcoinNetwork::Mainnet,
        ) {
            Ok(_) => (),
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": "Error providing master seed to node",
                }))
            }
        };

        let payment = match self
            .client
            .pay_uma_invoice(
                &self.config.node_id,
                &pay_req_data.encoded_invoice,
                60,
                1_000_000,
                None,
            )
            .await
        {
            Ok(payment) => payment,
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": "Failed to pay invoice",
                }))
            }
        };

        let payment = match self.wait_for_payment_completion(payment).await {
            Ok(payment) => payment,
            Err(e) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": format!("Failed to wait for payment completion: {}", e),
                }))
            }
        };

        println!(
            "Payment {} completed: {:?}",
            payment.id,
            payment.status.to_string()
        );

        let mut utxos_with_amounts = Vec::<UtxoWithAmount>::new();
        for utxo in payment.uma_post_transaction_data.iter().flatten() {
            let millsatoshi_amount = match value_millisatoshi(&utxo.amount) {
                Ok(amount) => amount,
                Err(_) => {
                    return HttpResponse::InternalServerError().json(json!({
                        "status": "ERROR",
                        "reason": "Error converting amount to millisatoshi",
                    }))
                }
            };
            utxos_with_amounts.push(UtxoWithAmount {
                utxo: utxo.utxo.clone(),
                amount: millsatoshi_amount,
            });
        }

        let utxo_with_amounts_bytes = match serde_json::to_vec(&utxos_with_amounts) {
            Ok(bytes) => bytes,
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": "Error serializing utxos",
                }))
            }
        };

        println!(
            "Sending UTXOs to {}: {}",
            pay_req_data.utxo_callback,
            String::from_utf8_lossy(&utxo_with_amounts_bytes)
        );

        HttpResponse::Ok().json(json!({
            "didSucceed": payment.status.to_string() == TransactionStatus::Success.to_string(),
            "paymentId": payment.id,
        }))
    }

    async fn wait_for_payment_completion(
        &self,
        payment: OutgoingPayment,
    ) -> Result<OutgoingPayment, Error> {
        let mut attemp_limit = 200;
        let mut payment = payment;
        while payment.status.to_string() != TransactionStatus::Success.to_string()
            && payment.status.to_string() != TransactionStatus::Failed.to_string()
            && payment.status.to_string() != TransactionStatus::Cancelled.to_string()
        {
            if attemp_limit == 0 {
                return Err(Error::PaymentTimeOut);
            }

            attemp_limit -= 1;
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

            payment = match self.client.get_entity::<OutgoingPayment>(&payment.id).await {
                Ok(p) => p,
                Err(e) => return Err(Error::LightsparkError(e)),
            };
        }

        Ok(payment)
    }

    pub fn handle_well_known_lnurlp(
        &self,
        request: &HttpRequest,
        username: &str,
    ) -> impl Responder {
        if username != self.config.username {
            return HttpResponse::NotFound().json(json!({
                "status": "ERROR",
                "reason": format!("User not found: {}", username),
            }));
        }

        let request_url = match Url::parse(request.uri().to_string().as_str()) {
            Ok(url) => url,
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "status": "ERROR",
                    "reason": "Error parsing request URL",
                }))
            }
        };

        if is_uma_lnurl_query(&request_url) {
            return HttpResponse::Ok().into();
        }
        unimplemented!()
    }

    pub fn handle_lnurl_payreq(&self, uuid: &str) -> String {
        format(format_args!("Hello {}!", uuid))
    }

    pub fn handle_uma_payreq(&self, uuid: &str) -> String {
        format(format_args!("Hello {}!", uuid))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PayerInfo {
    name: Option<String>,
    email: Option<String>,
    identifier: String,
}

fn get_payer_info(option: &PayerDataOptions) -> PayerInfo {
    let name = match option.name_required {
        true => Some("Alice FakeName".to_string()),
        false => None,
    };

    let email = match option.email_required {
        true => Some("$alice@vasp2.com".to_string()),
        false => None,
    };

    PayerInfo {
        name,
        email,
        identifier: "$alice@vasp1.com".to_string(),
    }
}
