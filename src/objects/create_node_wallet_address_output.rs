// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::types::entity_wrapper::EntityWrapper;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct CreateNodeWalletAddressOutput {
    #[serde(rename = "create_node_wallet_address_output_node")]
    pub node: EntityWrapper,

    #[serde(rename = "create_node_wallet_address_output_wallet_address")]
    pub wallet_address: String,
}

pub const FRAGMENT: &str = "
fragment CreateNodeWalletAddressOutputFragment on CreateNodeWalletAddressOutput {
    __typename
    create_node_wallet_address_output_node: node {
        id
    }
    create_node_wallet_address_output_wallet_address: wallet_address
}
";
