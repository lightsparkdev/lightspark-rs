
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::node_address_type::NodeAddressType;

/// This object represents the address of a node on the Lightning Network.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NodeAddress {

    /// The string representation of the address.
    #[serde (rename = "node_address_address")]
    pub address: String,

    /// The type, or protocol, of this address.
    #[serde (rename = "node_address_type")]
    pub _type: NodeAddressType,

}



pub const FRAGMENT: &str = "
fragment NodeAddressFragment on NodeAddress {
    __typename
    node_address_address: address
    node_address_type: type
}
";



