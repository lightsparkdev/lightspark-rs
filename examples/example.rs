// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use chrono::{Duration, Utc};
use lightspark::objects::bitcoin_network::BitcoinNetwork;
use lightspark::objects::channel_closing_transaction::ChannelClosingTransaction;
use lightspark::objects::channel_opening_transaction::ChannelOpeningTransaction;
use lightspark::objects::currency_amount::CurrencyAmount;
use lightspark::objects::deposit::Deposit;
use lightspark::objects::lightspark_node::LightsparkNode;
use lightspark::objects::outgoing_payment::OutgoingPayment;
use lightspark::objects::transaction::Transaction;
use lightspark::objects::withdrawal::Withdrawal;
use lightspark::objects::withdrawal_mode::WithdrawalMode;
use lightspark::{client::LightsparkClient, requester::auth_provider::AccountAuthProvider};
use serde_json::Value;
use std::any::Any;
use std::collections::HashMap;

fn print_fees(fees: Option<CurrencyAmount>) {
    if let Some(fee) = fees {
        println!(
            "        Paid {} {} in fees.",
            fee.preferred_currency_value_approx, fee.preferred_currency_unit
        );
    }
}

#[tokio::main]
async fn main() {
    // Setting up the account.
    let api_id = std::env::var("LIGHTSPARK_API_TOKEN_CLIENT_ID").unwrap();
    let api_token = std::env::var("LIGHTSPARK_API_TOKEN_CLIENT_SECRET").unwrap();

    let node_password = std::env::var("LIGHTSPARK_TEST_NODE_PASSWORD").unwrap();

    // Create LightsparkClient
    let auth_provider = AccountAuthProvider::new(api_id, api_token);
    let client = match LightsparkClient::new(auth_provider) {
        Ok(value) => value,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    // Bitcoin Fee estimate
    if let Ok(fee_estimate) = client
        .get_bitcoin_fee_estimates(BitcoinNetwork::Regtest)
        .await
    {
        println!(
            "Fees for a fast transaction {} {}",
            fee_estimate.fee_fast.preferred_currency_value_approx,
            fee_estimate.fee_fast.preferred_currency_unit
        );

        println!(
            "Fees for a cheap transaction {} {}",
            fee_estimate.fee_min.preferred_currency_value_approx,
            fee_estimate.fee_min.preferred_currency_unit
        );
    }
    println!();

    // Get current account
    let account = match client.get_current_account().await {
        Ok(v) => v,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    if let Some(name) = account.name.clone() {
        println!("You account name is {}", name);
    }

    // Get current account's API tokens
    let connection = match account.get_api_tokens(&client.requester, None).await {
        Ok(v) => v,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    println!(
        "You initially have {} active API token(s).",
        connection.count
    );
    println!();

    // Create a new API Token
    if let Ok(new_api_token) = client.create_api_token("Test token", true, true).await {
        println!("Created API token {}.", new_api_token.0.id);

        let connection = match account.get_api_tokens(&client.requester, None).await {
            Ok(v) => v,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };
        println!("You now have {} active API token(s).", connection.count);
        println!();

        // Delete the created API token
        match client.delete_api_token(new_api_token.0.id.as_str()).await {
            Ok(()) => println!("Deleted API token {}", new_api_token.0.id),
            Err(err) => {
                println!("{}", err);
                return;
            }
        };
    } else {
        println!("Creating API token error.");
    }

    let conductivity = account
        .get_conductivity(&client.requester, Some(vec![BitcoinNetwork::Regtest]), None)
        .await;
    if let Ok(Some(conductivity)) = conductivity {
        println!(
            "Your account's conductivity on REGTEST is {}/10.",
            conductivity
        );
        println!();
    }

    // Get account balance
    let local_balance = account
        .get_local_balance(&client.requester, Some(vec![BitcoinNetwork::Regtest]), None)
        .await;
    if let Ok(Some(local_balance)) = local_balance {
        println!(
            "Your local balance is {} {}",
            local_balance.preferred_currency_value_approx, local_balance.preferred_currency_unit
        )
    }

    let remote_balance = account
        .get_remote_balance(&client.requester, Some(vec![BitcoinNetwork::Regtest]), None)
        .await;
    if let Ok(Some(remote_balance)) = remote_balance {
        println!(
            "Your remote balance is {} {}",
            remote_balance.preferred_currency_value_approx, remote_balance.preferred_currency_unit
        )
    }

    println!();

    // Get nodes in the account.
    let node_connections = match account
        .get_nodes(
            &client.requester,
            Some(50),
            Some(vec![BitcoinNetwork::Regtest]),
            None,
        )
        .await
    {
        Ok(v) => v,
        Err(_) => panic!("Unable to fetch the nodes"),
    };
    println!("You have {} node(s).", node_connections.count);

    let mut node_id: Option<String> = None;
    let mut node_name: Option<String> = None;
    println!("{}", node_connections.entities.len());
    for node in node_connections.entities {
        println!("node info");
        node_name = Some(node.display_name.clone());
        node_id = Some(node.id.clone());
        println!("{} {}", node.display_name, node.id);
    }
    println!();

    let node_id = node_id.unwrap();
    let node_name = node_name.unwrap();

    // Fund node in test mode.
    match client.fund_node(node_id.as_str(), 10000).await {
        Ok(amount) => {
            println!(
                "Found {} {} to {}",
                amount.preferred_currency_value_approx, amount.preferred_currency_unit, node_name
            );
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    // Get transactions in the account
    let transactions_connection = match account
        .get_transactions(
            &client.requester,
            Some(30),
            None,
            None,
            None,
            None,
            Some(BitcoinNetwork::Regtest),
            None,
            None,
            None,
        )
        .await
    {
        Ok(v) => v,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    println!(
        "There is a total of {} transaction(s) on this account:",
        transactions_connection.count
    );

    let mut deposit_transaction_id: Option<String> = None;
    for transaction in transactions_connection.entities {
        let type_name = Transaction::type_name(transaction.as_ref());
        println!(
            "    - {} at {}: {} {} ({})",
            type_name,
            transaction.get_created_at(),
            transaction.get_amount().preferred_currency_value_approx,
            transaction.get_amount().preferred_currency_unit,
            transaction.get_status()
        );

        if type_name == "Deposit" {
            deposit_transaction_id = Some(transaction.get_id().clone());
        }

        let transaction: Box<dyn Any> = Box::new(transaction);
        if let Some(outgoing_payment) = transaction.downcast_ref::<OutgoingPayment>() {
            print_fees(outgoing_payment.fees.clone());
        } else if let Some(deposit) = transaction.downcast_ref::<Deposit>() {
            print_fees(deposit.fees.clone());
        } else if let Some(withdrawal) = transaction.downcast_ref::<Withdrawal>() {
            print_fees(withdrawal.fees.clone());
        } else if let Some(opening) = transaction.downcast_ref::<ChannelOpeningTransaction>() {
            print_fees(opening.fees.clone());
        } else if let Some(closing) = transaction.downcast_ref::<ChannelClosingTransaction>() {
            print_fees(closing.fees.clone());
        }
    }

    println!();

    // Pagination
    let page_size = 10;
    let mut iterations = 0;
    let mut has_next = true;
    let mut after: Option<String> = None;
    while has_next && iterations < 30 {
        iterations += 1;
        let transactions_connection = match account
            .get_transactions(
                &client.requester,
                Some(page_size),
                after.clone(),
                None,
                None,
                None,
                Some(BitcoinNetwork::Regtest),
                None,
                None,
                None,
            )
            .await
        {
            Ok(v) => v,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };

        let num = transactions_connection.entities.len();
        println!(
            "We got {} transactions for the page (iteration #{})",
            num, iterations
        );

        if transactions_connection.page_info.has_next_page.unwrap() {
            has_next = true;
            after = transactions_connection.page_info.end_cursor;
            println!("  And we have another page!")
        } else {
            has_next = false;
            println!("  And we're done!")
        }
    }
    println!();

    // Get transactions in the past 24 hours
    let time = Utc::now() - Duration::hours(24);
    let transactions_connection = match account
        .get_transactions(
            &client.requester,
            None,
            None,
            None,
            Some(time),
            None,
            Some(BitcoinNetwork::Regtest),
            None,
            None,
            None,
        )
        .await
    {
        Ok(v) => v,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    println!(
        "We had {} transactions in the past 24 hours.",
        transactions_connection.count
    );

    // Get a transaction detail.
    if let Some(deposit_transaction_id) = deposit_transaction_id {
        let deposit: Deposit = match client
            .get_entity::<Deposit>(deposit_transaction_id.as_str())
            .await
        {
            Ok(v) => v,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };
        println!("Details of deposit transaction");
        println!("id: {}", deposit.id);
        println!(
            "amount: {} {}",
            deposit.amount.preferred_currency_value_approx, deposit.amount.preferred_currency_unit
        );
        println!("created at: {}", deposit.created_at);
        println!("updated at: {}", deposit.updated_at);
        println!();
    }

    // Create a lightning invoice
    let invoice = match client
        .create_invoice(&node_id, 42000, Some("Pizza"), None)
        .await
    {
        Ok(v) => v,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    println!("Invoice created from {}:", node_name);
    println!(
        "Encoded invoice = {}",
        invoice.data.encoded_payment_request.clone()
    );
    println!();

    // Decode a payment request
    let decoded_request = match client
        .get_decoded_payment_request(invoice.data.encoded_payment_request.as_str())
        .await
    {
        Ok(v) => v,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    println!("Decoded payment request:");
    println!(
        "    destination_public_key = {}",
        decoded_request.destination.get_public_key().unwrap()
    );
    println!(
        "    amount = {} {}",
        decoded_request.amount.preferred_currency_value_approx,
        decoded_request.amount.preferred_currency_unit
    );
    if let Some(memo) = decoded_request.memo {
        println!("    memo = {}", memo);
    }
    println!();

    // Unlock the node
    match client
        .recover_node_signing_key(node_id.as_str(), node_password.as_str())
        .await
    {
        Ok(v) => {
            println!("{}'s signing key has been loaded.", node_name);
            v
        }
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    // Lightning Fee Estimate
    //
    // match client
    //     .get_lightning_fee_estimate_for_invoice(
    //         node_id.as_str(),
    //         /* Encoded Invoice */,
    //         /* Amount */,
    //     )
    //     .await
    // {
    //     Ok(amount) => {
    //         println!(
    //             "Estimate fee for paying this invoice is {} {}",
    //             amount.preferred_currency_value_approx, amount.preferred_currency_unit
    //         );
    //     }
    //     Err(err) => {
    //         println!("{}", err);
    //     }
    // };

    // Pay Invoice
    //
    // let payment = match client
    //     .pay_invoice(
    //         node_id.as_str(),
    //         /* Encoded Invoice */,
    //         60,
    //         None,
    //         /* Payment Amount */,
    //     )
    //     .await
    // {
    //     Ok(v) => v,
    //     Err(err) => {
    //         println!("{}", err);
    //         return;
    //     }
    // };

    // Key Send
    //
    // match client
    //     .get_lightning_fee_estimate_for_node(
    //        node_id.as_str(),
    //        /* Node Public Key */,
    //        500000)
    //     .await
    // {
    //     Ok(amount) => {
    //         println!(
    //             "Estimate fee for paying this node is {} {}",
    //             amount.preferred_currency_value_approx, amount.preferred_currency_unit
    //         );
    //     }
    //     Err(err) => {
    //         println!("{}", err);
    //     }
    // };

    // Key Send
    //
    // let payment = match client
    //     .send_payment(
    //         node_id.as_str(),
    //         /* Node Public Key */,
    //         60,
    //         2000000,
    //         500,
    //     )
    //     .await
    // {
    //     Ok(v) => v,
    //     Err(err) => {
    //         println!("{}", err);
    //         return;
    //     }
    // };
    // println!(
    //     "Payment directly to node without invoice done with ID = {}",
    //     payment.id
    // );
    // println!();

    let address = match client.create_node_wallet_address(&node_id).await {
        Ok(v) => v,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    println!("Got a bitcoin address for {}: {}", node_name, address);
    println!();

    if let Ok(withdrawal_request) = client
        .request_withdrawal(
            node_id.as_str(),
            address.as_str(),
            1000,
            WithdrawalMode::WalletOnly,
        )
        .await
    {
        println!(
            "Money was withdrawn with request ID = {}",
            withdrawal_request.id
        );
        println!();
    }

    // Fetch the channels for the node
    let node = match client.get_entity::<LightsparkNode>(node_id.as_str()).await {
        Ok(v) => v,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    if let Ok(channels_connection) = node.get_channels(&client.requester, Some(10), None).await {
        println!(
            "{} has {} channel(s):",
            node_name, channels_connection.count
        );
        for channel in channels_connection.entities {
            if let Some(node_entity) = channel.remote_node {
                if let Ok(remote_node) = client
                    .get_entity::<LightsparkNode>(node_entity.id.as_str())
                    .await
                {
                    let alias = remote_node.alias.unwrap_or("UNKNOWN".to_owned());
                    if let Some(local_balance) = channel.local_balance {
                        if let Some(remote_balance) = channel.remote_balance {
                            println!(
                                "    - With {}. Local/remote balance = {} {} {}",
                                alias,
                                local_balance.preferred_currency_value_approx,
                                remote_balance.preferred_currency_value_approx,
                                remote_balance.preferred_currency_unit
                            );
                        }
                    }
                }
            }
        }
    }
    println!();

    // Execute a custom graphql operation
    let mut variables: HashMap<&str, Value> = HashMap::new();
    variables.insert("networks", BitcoinNetwork::Regtest.into());

    let result = match client
        .execute_graphql_request(
            "query Test($networks: [BitcoinNetwork!]!) {
            current_account {
                name
                nodes(bitcoin_networks: $networks) {
                    count
                }
            }
        }",
            variables,
        )
        .await
    {
        Ok(v) => v,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let name = result["current_account"]["name"].clone();
    let count = result["current_account"]["nodes"]["count"].clone();
    println!(
        "The account {} has {} nodes on the REGTEST network.",
        name, count
    );
}
