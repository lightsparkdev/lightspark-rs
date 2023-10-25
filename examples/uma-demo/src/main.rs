pub mod config;
pub mod vasp;
use std::sync::Arc;

use actix_web::{get, post, web, App, HttpServer, Responder};

use crate::vasp::{VASPReceiving, VASPSending, VASP};

#[get("/api/umalookup/{receiver}")]
async fn uma_lookup(vasp: web::Data<Arc<VASP>>, receiver: web::Path<String>) -> impl Responder {
    vasp.handle_client_uma_lookup(receiver.as_str())
}

#[get("/api/umapayreq/{callback_uuid}")]
async fn client_payreq(
    vasp: web::Data<Arc<VASP>>,
    callback_uuid: web::Path<String>,
) -> impl Responder {
    vasp.handle_client_pay_req(callback_uuid.as_str())
}

#[get("/api/sendpayment/{callback_uuid}")]
async fn send_payment(
    vasp: web::Data<Arc<VASP>>,
    callback_uuid: web::Path<String>,
) -> impl Responder {
    vasp.handle_client_payment_confirm(callback_uuid.as_str())
}

#[get("/.well-known/lnurlp/{username}")]
async fn well_known_lnurlp(
    vasp: web::Data<Arc<VASP>>,
    username: web::Path<String>,
) -> impl Responder {
    vasp.handle_well_known_lnurlp(username.as_str())
}

#[get("/api/uma/payreq/{uuid}")]
async fn lnurl_payreq(vasp: web::Data<Arc<VASP>>, uuid: web::Path<String>) -> impl Responder {
    vasp.handle_lnurl_payreq(uuid.as_str())
}

#[post("/api/uma/payreq/{uuid}")]
async fn uma_payreq(vasp: web::Data<Arc<VASP>>, uuid: web::Path<String>) -> impl Responder {
    vasp.handle_uma_payreq(uuid.as_str())
}

#[get("/.well-known/lnurlpubkey")]
async fn pubkey_request(vasp: web::Data<Arc<VASP>>) -> impl Responder {
    vasp.handle_pubkey_request()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("VASP_SERVER_PORT")
        .unwrap()
        .parse::<u16>()
        .unwrap();
    let vasp = Arc::new(VASP {
        config: config::Config::new_from_env(),
    });
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
