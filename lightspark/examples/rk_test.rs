use lightspark::{
    client::LightsparkClient, key::Secp256k1SigningKey, request::auth_provider::AccountAuthProvider,
};

async fn create_invoice() {
    let api_id = std::env::var("RK_API_CLIENT_ID").unwrap();
    let api_token = std::env::var("RK_API_CLIENT_SECRET").unwrap();
    let endpoint = std::env::var("RK_API_ENDPOINT").unwrap();

    let auth = AccountAuthProvider::new(api_id.to_string(), api_token.to_string());
    let mut client = LightsparkClient::<Secp256k1SigningKey>::new(auth).unwrap();
    client.requester.set_base_url(Some(endpoint));

    let node_id = std::env::var("RK_NODE_ID").unwrap();

    let master_seed = std::env::var("RK_MASTER_SEED_HEX").unwrap();
    let _ = client.provide_master_seed(
        &node_id,
        hex::decode(master_seed).unwrap(),
        lightspark::objects::bitcoin_network::BitcoinNetwork::Mainnet,
    );

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
    let api_id = std::env::var("RK_API_CLIENT_ID").unwrap();
    let api_token = std::env::var("RK_API_CLIENT_SECRET").unwrap();
    let endpoint = std::env::var("RK_API_ENDPOINT").unwrap();

    let auth = AccountAuthProvider::new(api_id.to_string(), api_token.to_string());
    let mut client = LightsparkClient::<Secp256k1SigningKey>::new(auth).unwrap();
    client.requester.set_base_url(Some(endpoint));

    let node_id = std::env::var("RK_NODE_ID").unwrap();

    let _ = client.fund_node(&node_id, 1000000).await;
    let master_seed = std::env::var("RK_MASTER_SEED_HEX").unwrap();
    let _ = client.provide_master_seed(
        &node_id,
        hex::decode(master_seed).unwrap(),
        lightspark::objects::bitcoin_network::BitcoinNetwork::Mainnet,
    );

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
