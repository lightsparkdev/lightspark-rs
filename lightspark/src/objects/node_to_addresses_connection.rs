// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::node_address::NodeAddress;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

/// A connection between a node and the addresses it has announced for itself on Lightning Network.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NodeToAddressesConnection {
    /// The total count of objects in this connection, using the current filters. It is different from the number of objects returned in the current page (in the `entities` field).
    #[serde(rename = "node_to_addresses_connection_count")]
    pub count: i64,

    /// The addresses for the current page of this connection.
    #[serde(rename = "node_to_addresses_connection_entities")]
    pub entities: Vec<NodeAddress>,
}

pub const FRAGMENT: &str = "
fragment NodeToAddressesConnectionFragment on NodeToAddressesConnection {
    __typename
    node_to_addresses_connection_count: count
    node_to_addresses_connection_entities: entities {
        __typename
        node_address_address: address
        node_address_type: type
    }
}
";
