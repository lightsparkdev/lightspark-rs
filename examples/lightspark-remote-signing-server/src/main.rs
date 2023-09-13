use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use lightspark::{
    client::LightsparkClient,
    key::{OperationSigningKey, Secp256k1SigningKey},
    request::auth_provider::AccountAuthProvider,
    webhooks::{self, WebhookEvent},
};
use lightspark_remote_signing::{
    handler::Handler,
    signer::{self, LightsparkSigner, Seed},
    validation::{PositiveValidator, Validation},
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let seed = Seed::new("test".as_bytes().to_vec());
    let signer = LightsparkSigner::new(&seed, signer::Network::Bitcoin).unwrap();
    let validator = PositiveValidator;
    let handler = Handler::new(signer, validator);

    let auth = AccountAuthProvider::new("test".to_owned(), "test".to_owned());
    let client = LightsparkClient::<Secp256k1SigningKey>::new(auth).expect("Assume success");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &handler, &client).await;
    }
}

async fn handle_connection<T: Validation, K: OperationSigningKey>(
    stream: TcpStream,
    handler: &Handler<T>,
    client: &LightsparkClient<K>,
) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut name = String::new();
    loop {
        let r = reader.read_line(&mut name).unwrap();
        if r < 3 {
            break;
        }
    }
    let mut size = 0;
    let linesplit = name.split('\n');
    let mut sig = "";
    for l in linesplit {
        if l.starts_with("Content-Length") {
            let sizeplit = l.split(':');
            for s in sizeplit {
                if !(s.starts_with("Content-Length")) {
                    size = s.trim().parse::<usize>().unwrap();
                }
            }
        } else if l.starts_with(webhooks::SIGNATURE_HEADER) {
            let sigsplit = l.split(':');
            for s in sigsplit {
                if !(s.starts_with(webhooks::SIGNATURE_HEADER)) {
                    sig = s.trim();
                }
            }
        }
    }
    let mut buffer = vec![0; size];
    reader.read_exact(&mut buffer).unwrap();
    let body = String::from_utf8(buffer.clone()).unwrap();

    println!("Signature: {}", sig);
    println!("Request: {}", body);

    let event = WebhookEvent::verify_and_parse(
        buffer.as_slice(),
        sig.to_string(),
        "webhook_secret".to_owned(),
    )
    .expect("Assume verified");
    let response = handler.handle_remote_signing_webhook_msg(&event).expect("");
    let _ = client
        .execute_graphql_request_variable(&response.query, response.variables.clone())
        .await;
}
