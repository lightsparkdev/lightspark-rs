pub mod config;
pub mod vasp;
pub mod vasp_request_cache;
use std::sync::{Arc, Mutex};

use actix_web::{get, post, web, App, HttpRequest, HttpServer, Responder};
use lightspark::{
    client::LightsparkClient, key::Secp256k1SigningKey, request::auth_provider::AccountAuthProvider,
};
use serde::Deserialize;
use uma::public_key_cache::InMemoryPublicKeyCache;

use crate::vasp::SendingVASP;

#[get("/api/umalookup/{receiver}")]
async fn uma_lookup(
    vasp: web::Data<Arc<Mutex<SendingVASP<InMemoryPublicKeyCache>>>>,
    receiver: web::Path<String>,
) -> impl Responder {
    let mut vasp = vasp.lock().unwrap();
    vasp.handle_client_uma_lookup(receiver.as_str())
}

#[derive(Deserialize)]
struct PayReqParam {
    amount: i64,

    #[serde(rename = "currencyCode")]
    currency_code: String,
}

#[get("/api/umapayreq/{callback_uuid}")]
#[allow(clippy::await_holding_lock)]
async fn client_payreq(
    vasp: web::Data<Arc<Mutex<SendingVASP<InMemoryPublicKeyCache>>>>,
    callback_uuid: web::Path<String>,
    params: web::Query<PayReqParam>,
) -> impl Responder {
    let mut vasp = vasp.lock().unwrap();
    vasp.handle_client_pay_req(
        callback_uuid.as_str(),
        params.amount,
        params.currency_code.as_str(),
    )
    .await
}

#[get("/api/sendpayment/{callback_uuid}")]
#[allow(clippy::await_holding_lock)]
async fn send_payment(
    vasp: web::Data<Arc<Mutex<SendingVASP<InMemoryPublicKeyCache>>>>,
    callback_uuid: web::Path<String>,
) -> impl Responder {
    let mut vasp = vasp.lock().unwrap();
    vasp.handle_client_payment_confirm(callback_uuid.as_str())
        .await
}

#[get("/.well-known/lnurlp/{username}")]
async fn well_known_lnurlp(
    req: HttpRequest,
    vasp: web::Data<Arc<SendingVASP<InMemoryPublicKeyCache>>>,
    username: web::Path<String>,
) -> impl Responder {
    vasp.handle_well_known_lnurlp(&req, username.as_str())
}

#[get("/api/uma/payreq/{uuid}")]
async fn lnurl_payreq(
    vasp: web::Data<Arc<SendingVASP<InMemoryPublicKeyCache>>>,
    uuid: web::Path<String>,
) -> impl Responder {
    vasp.handle_lnurl_payreq(uuid.as_str())
}

#[post("/api/uma/payreq/{uuid}")]
async fn uma_payreq(
    vasp: web::Data<Arc<SendingVASP<InMemoryPublicKeyCache>>>,
    uuid: web::Path<String>,
) -> impl Responder {
    vasp.handle_uma_payreq(uuid.as_str())
}

#[get("/.well-known/lnurlpubkey")]
async fn pubkey_request(
    vasp: web::Data<Arc<SendingVASP<InMemoryPublicKeyCache>>>,
) -> impl Responder {
    vasp.handle_pubkey_request()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("VASP_SERVER_PORT")
        .unwrap()
        .parse::<u16>()
        .unwrap();
    let pubkey_cache = InMemoryPublicKeyCache::new();
    let req_cache = vasp_request_cache::Vasp1PayReqCache::new();
    let config = config::Config::new_from_env();
    let auth_provider = AccountAuthProvider::new(
        config.api_client_id.clone(),
        config.api_client_secret.clone(),
    );
    let lightspark_client = LightsparkClient::<Secp256k1SigningKey>::new(auth_provider).unwrap();

    let vasp = Arc::new(Mutex::new(SendingVASP {
        config: config::Config::new_from_env(),
        pubkey_cache,
        request_cache: req_cache,
        client: lightspark_client,
    }));

    HttpServer::new(move || {
        let vasp = Arc::clone(&vasp);
        App::new()
            .app_data(vasp)
            .service(uma_lookup)
            .service(client_payreq)
            .service(send_payment)
            .service(well_known_lnurlp)
            .service(lnurl_payreq)
            .service(uma_payreq)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
