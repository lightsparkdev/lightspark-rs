use actix_web::{
    get, middleware, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use futures_util::StreamExt as _;
use lightspark::key::Secp256k1SigningKey;
use lightspark::request::auth_provider::AccountAuthProvider;
use lightspark_remote_signing::lightspark::webhooks::WebhookEvent;
use lightspark_remote_signing::{
    handler::Handler,
    signer::{LightsparkSigner, Seed},
    validation::PositiveValidator,
};
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

pub mod config;

#[get("/ping")]
async fn ping() -> impl Responder {
    info!("ping");
    HttpResponse::NoContent().finish()
}

#[post("/ln/webhooks")]
async fn webhook_handler(
    req: HttpRequest,
    mut body: web::Payload,
    data: web::Data<config::Config>,
) -> impl Responder {
    let headers = req.headers();
    let signature = headers
        .get(lightspark_remote_signing::lightspark::webhooks::SIGNATURE_HEADER)
        .unwrap();
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item.unwrap());
    }

    let auth = AccountAuthProvider::new(data.api_client_id.clone(), data.api_client_secret.clone());
    let mut client =
        lightspark::client::LightsparkClient::<Secp256k1SigningKey>::new(auth).unwrap();
    client.requester.set_base_url(data.api_endpoint.clone());

    let seed = Seed::new(hex::decode(data.master_seed_hex.clone()).unwrap());
    let signer =
        LightsparkSigner::new(&seed, lightspark_remote_signing::signer::Network::Regtest).unwrap();
    let validation = PositiveValidator;
    let handler = Handler::new(signer, Box::new(validation));

    let event =
        WebhookEvent::verify_and_parse(&bytes, signature.to_str().unwrap(), &data.webhook_secret)
            .unwrap();
    let response = handler
        .handle_remote_signing_webhook_msg(&event)
        .unwrap()
        .unwrap();

    debug!("Response {:?}", response);

    let result = client
        .execute_graphql_request_variable(&response.query, response.variables)
        .await;

    debug!("Graphql response {:?}", result);
    HttpResponse::NoContent().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let config = config::Config::new_from_env();
    let port = config.api_port;

    info!(config = format!("{:?}", config), "Starting Remote Signer.");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .wrap(middleware::NormalizePath::trim())
            .service(ping)
            .service(webhook_handler)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
