// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::error::Error;
use crate::objects::bitcoin_network::BitcoinNetwork;
use crate::objects::entity::Entity;
use crate::objects::node::Node;
use crate::objects::node_address_type::NodeAddressType;
use crate::objects::node_to_addresses_connection::NodeToAddressesConnection;
use crate::requester::requester::Requester;
use crate::types::custom_date_format::custom_date_format;
use crate::types::get_entity::GetEntity;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::vec::Vec;

/// This is a node on the Lightning Network, managed by a third party. The information about this node is public data that has been obtained by observing the Lightning Network.
#[derive(Deserialize)]
pub struct GraphNode {
    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    #[serde(rename = "graph_node_id")]
    pub id: String,

    /// The date and time when the entity was first created.
    #[serde(with = "custom_date_format", rename = "graph_node_created_at")]
    pub created_at: DateTime<Utc>,

    /// The date and time when the entity was last updated.
    #[serde(with = "custom_date_format", rename = "graph_node_updated_at")]
    pub updated_at: DateTime<Utc>,

    /// A name that identifies the node. It has no importance in terms of operating the node, it is just a way to identify and search for commercial services or popular nodes. This alias can be changed at any time by the node operator.
    #[serde(rename = "graph_node_alias")]
    pub alias: Option<String>,

    /// The Bitcoin Network this node is deployed in.
    #[serde(rename = "graph_node_bitcoin_network")]
    pub bitcoin_network: BitcoinNetwork,

    /// A hexadecimal string that describes a color. For example "#000000" is black, "#FFFFFF" is white. It has no importance in terms of operating the node, it is just a way to visually differentiate nodes. That color can be changed at any time by the node operator.
    #[serde(rename = "graph_node_color")]
    pub color: Option<String>,

    /// A summary metric used to capture how well positioned a node is to send, receive, or route transactions efficiently. Maximizing a node's conductivity helps a node’s transactions to be capital efficient. The value is an integer ranging between 0 and 10 (bounds included).
    #[serde(rename = "graph_node_conductivity")]
    pub conductivity: Option<i64>,

    /// The name of this node in the network. It will be the most human-readable option possible, depending on the data available for this node.
    #[serde(rename = "graph_node_display_name")]
    pub display_name: String,

    /// The public key of this node. It acts as a unique identifier of this node in the Lightning Network.
    #[serde(rename = "graph_node_public_key")]
    pub public_key: Option<String>,
}

impl Node for GraphNode {
    /// A name that identifies the node. It has no importance in terms of operating the node, it is just a way to identify and search for commercial services or popular nodes. This alias can be changed at any time by the node operator.
    fn get_alias(&self) -> Option<String> {
        return self.alias.clone();
    }

    /// The Bitcoin Network this node is deployed in.
    fn get_bitcoin_network(&self) -> BitcoinNetwork {
        return self.bitcoin_network.clone();
    }

    /// A hexadecimal string that describes a color. For example "#000000" is black, "#FFFFFF" is white. It has no importance in terms of operating the node, it is just a way to visually differentiate nodes. That color can be changed at any time by the node operator.
    fn get_color(&self) -> Option<String> {
        return self.color.clone();
    }

    /// A summary metric used to capture how well positioned a node is to send, receive, or route transactions efficiently. Maximizing a node's conductivity helps a node’s transactions to be capital efficient. The value is an integer ranging between 0 and 10 (bounds included).
    fn get_conductivity(&self) -> Option<i64> {
        return self.conductivity;
    }

    /// The name of this node in the network. It will be the most human-readable option possible, depending on the data available for this node.
    fn get_display_name(&self) -> String {
        return self.display_name.clone();
    }

    /// The public key of this node. It acts as a unique identifier of this node in the Lightning Network.
    fn get_public_key(&self) -> Option<String> {
        return self.public_key.clone();
    }

    fn type_name(&self) -> &'static str {
        "GraphNode"
    }
}

impl Entity for GraphNode {
    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    fn get_id(&self) -> String {
        return self.id.clone();
    }

    /// The date and time when the entity was first created.
    fn get_created_at(&self) -> DateTime<Utc> {
        return self.created_at;
    }

    /// The date and time when the entity was last updated.
    fn get_updated_at(&self) -> DateTime<Utc> {
        return self.updated_at;
    }

    fn type_name(&self) -> &'static str {
        "GraphNode"
    }
}

impl GetEntity for GraphNode {
    fn get_entity_query() -> String {
        return format!(
            "
        query GetEntity($id: ID!) {{
            entity(id: $id) {{
                ... on GraphNode {{
                    ... GraphNodeFragment
                }}
            }}
        }}

        {}",
            FRAGMENT
        );
    }
}

pub const FRAGMENT: &str = "
fragment GraphNodeFragment on GraphNode {
    __typename
    graph_node_id: id
    graph_node_created_at: created_at
    graph_node_updated_at: updated_at
    graph_node_alias: alias
    graph_node_bitcoin_network: bitcoin_network
    graph_node_color: color
    graph_node_conductivity: conductivity
    graph_node_display_name: display_name
    graph_node_public_key: public_key
}
";

impl GraphNode {
    pub async fn get_addresses(
        &self,
        requester: &Requester,
        first: Option<i64>,
        types: Option<Vec<NodeAddressType>>,
    ) -> Result<NodeToAddressesConnection, Error> {
        let query = "query FetchNodeToAddressesConnection($entity_id: ID!, $first: Int, $types: [NodeAddressType!]) {
    entity(id: $entity_id) {
        ... on GraphNode {
            addresses(, first: $first, types: $types) {
                __typename
                node_to_addresses_connection_count: count
                node_to_addresses_connection_entities: entities {
                    __typename
                    node_address_address: address
                    node_address_type: type
                }
            }
        }
    }
}";
        let mut variables: HashMap<&str, Value> = HashMap::new();
        variables.insert("entity_id", self.id.clone().into());
        variables.insert("first", first.into());
        variables.insert("types", types.into());

        let value = serde_json::to_value(variables).map_err(|err| Error::ConversionError(err))?;
        let result = requester
            .execute_graphql(&query, Some(value))
            .await
            .map_err(|err| Error::ClientError(err))?;
        let json = result["entity"]["addresses"].clone();
        let result = serde_json::from_value(json).map_err(|err| Error::JsonError(err))?;
        Ok(result)
    }
}
