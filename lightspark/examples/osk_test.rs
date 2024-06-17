use lightspark::{
    client::LightsparkClient, key::RSASigningKey, request::auth_provider::AccountAuthProvider,
};

async fn create_invoice() {
    let api_id = std::env::var("LIGHTSPARK_API_CLIENT_ID").unwrap();
    let api_token = std::env::var("LIGHTSPARK_API_CLIENT_SECRET").unwrap();
    let endpoint = std::env::var("LIGHTSPARK_API_ENDPOINT").unwrap();

    let auth = AccountAuthProvider::new(api_id.to_string(), api_token.to_string());
    let mut client = LightsparkClient::<RSASigningKey>::new(auth).unwrap();
    client.requester.set_base_url(Some(endpoint));

    let node_id = std::env::var("LIGHTSPARK_NODE_ID").unwrap();

    let password = std::env::var("LIGHTSPARK_NODE_PASSWORD").unwrap();
    let _ = client.recover_node_signing_key(&node_id, &password).await;

    println!("API ID: {:?}", api_id);
    println!("API Token: {:?}", api_token);
    println!("Node ID: {:?}", node_id);

    let account = client.get_current_account().await.unwrap();
    println!("Account: {:?}", account.name);

    let invoice = client.create_invoice(&node_id, 10000, None, None).await;
    let payment_request = invoice.unwrap().data.encoded_payment_request;
    println!("Invoice created: {:?}", payment_request);

    let response = client
        .create_test_mode_payment(&node_id, &payment_request, None)
        .await;
    println!("Payment response: {:?}", response.unwrap().id);
}

async fn test_payment() {
    let api_id = std::env::var("LIGHTSPARK_API_CLIENT_ID").unwrap();
    let api_token = std::env::var("LIGHTSPARK_API_CLIENT_SECRET").unwrap();
    let endpoint = std::env::var("LIGHTSPARK_API_ENDPOINT").unwrap();

    let auth = AccountAuthProvider::new(api_id.to_string(), api_token.to_string());
    let mut client = LightsparkClient::<RSASigningKey>::new(auth).unwrap();
    client.requester.set_base_url(Some(endpoint));

    let node_id = std::env::var("LIGHTSPARK_NODE_ID").unwrap();

    let password = std::env::var("LIGHTSPARK_NODE_PASSWORD").unwrap();
    let _ = client.recover_node_signing_key(&node_id, &password).await;

    println!("API ID: {:?}", api_id);
    println!("API Token: {:?}", api_token);
    println!("Node ID: {:?}", node_id);

    let account = client.get_current_account().await.unwrap();
    println!("Account: {:?}", account.name);

    let invoice = client
        .create_test_mode_invoice(&node_id, 10000, Some("test"), None)
        .await;
    let payment_request = invoice.unwrap().replace('\"', "");
    println!("Invoice created: {:?}", payment_request);

    let response = client
        .pay_invoice(&node_id, &payment_request, 100, None, 1000)
        .await;
    println!("Payment response: {:?}", response.unwrap().id);
}

#[tokio::main]
async fn main() {
    create_invoice().await;
    test_payment().await;
}
