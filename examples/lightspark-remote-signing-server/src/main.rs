use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use futures_util::StreamExt as _;
use lightspark::{
    key::Secp256k1SigningKey, request::auth_provider::AccountAuthProvider, webhooks::WebhookEvent,
};
use lightspark_remote_signing::{
    handler::Handler,
    signer::{LightsparkSigner, Seed},
    validation::PositiveValidator,
};

pub mod config;

#[get("/ping")]
async fn ping() -> impl Responder {
    println!("ping");
    HttpResponse::NoContent().finish()
}

#[post("/ln/webhooks")]
async fn webhook_handler(
    req: HttpRequest,
    mut body: web::Payload,
    data: web::Data<config::Config>,
) -> impl Responder {
    let headers = req.headers();
    let signature = headers.get(lightspark::webhooks::SIGNATURE_HEADER).unwrap();
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item.unwrap());
    }

    let auth = AccountAuthProvider::new(data.api_client_id.clone(), data.api_client_secret.clone());
    let client = lightspark::client::LightsparkClient::<Secp256k1SigningKey>::new(auth).unwrap();

    let seed = Seed::new(hex::decode(data.master_seed_hex.clone()).unwrap());
    let signer =
        LightsparkSigner::new(&seed, lightspark_remote_signing::signer::Network::Regtest).unwrap();
    let validation = PositiveValidator;
    let handler = Handler::new(signer, validation);

    let event =
        WebhookEvent::verify_and_parse(&bytes, signature.to_str().unwrap(), &data.webhook_secret)
            .unwrap();
    let response = handler.handle_remote_signing_webhook_msg(&event).unwrap();

    println!("Response {:?}", response);

    let result = client
        .execute_graphql_request_variable(&response.query, response.variables)
        .await;

    println!("Graphql response {:?}", result);
    HttpResponse::NoContent().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::new_from_env();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .service(ping)
            .service(webhook_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}