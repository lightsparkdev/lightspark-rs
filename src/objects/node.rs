// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use super::graph_node::GraphNode;
use super::lightspark_node_with_o_s_k::LightsparkNodeWithOSK;
use super::lightspark_node_with_remote_signing::LightsparkNodeWithRemoteSigning;
use crate::objects::bitcoin_network::BitcoinNetwork;
use crate::objects::entity::Entity;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub trait Node: Entity {
    /// A name that identifies the node. It has no importance in terms of operating the node, it is just a way to identify and search for commercial services or popular nodes. This alias can be changed at any time by the node operator.
    fn get_alias(&self) -> Option<String>;

    /// The Bitcoin Network this node is deployed in.
    fn get_bitcoin_network(&self) -> BitcoinNetwork;

    /// A hexadecimal string that describes a color. For example "#000000" is black, "#FFFFFF" is white. It has no importance in terms of operating the node, it is just a way to visually differentiate nodes. That color can be changed at any time by the node operator.
    fn get_color(&self) -> Option<String>;

    /// A summary metric used to capture how well positioned a node is to send, receive, or route transactions efficiently. Maximizing a node's conductivity helps a node’s transactions to be capital efficient. The value is an integer ranging between 0 and 10 (bounds included).
    fn get_conductivity(&self) -> Option<i64>;

    /// The name of this node in the network. It will be the most human-readable option possible, depending on the data available for this node.
    fn get_display_name(&self) -> String;

    /// The public key of this node. It acts as a unique identifier of this node in the Lightning Network.
    fn get_public_key(&self) -> Option<String>;

    fn type_name(&self) -> &'static str;
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum NodeEnum {
    GraphNode(GraphNode),
    LightsparkNodeWithOSK(LightsparkNodeWithOSK),
    LightsparkNodeWithRemoteSigning(LightsparkNodeWithRemoteSigning),
}

impl<'de> Deserialize<'de> for NodeEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        if let Some(typename) = value.get("__typename").and_then(Value::as_str) {
            match typename {
                "GraphNode" => {
                    let obj = GraphNode::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(NodeEnum::GraphNode(obj))
                }
                "LightsparkNodeWithOSK" => {
                    let obj = LightsparkNodeWithOSK::deserialize(value).map_err(|err| {
                        serde::de::Error::custom(format!("Serde JSON Error {}", err))
                    })?;
                    Ok(NodeEnum::LightsparkNodeWithOSK(obj))
                }
                "LightsparkNodeWithRemoteSigning" => {
                    let obj =
                        LightsparkNodeWithRemoteSigning::deserialize(value).map_err(|err| {
                            serde::de::Error::custom(format!("Serde JSON Error {}", err))
                        })?;
                    Ok(NodeEnum::LightsparkNodeWithRemoteSigning(obj))
                }

                _ => Err(serde::de::Error::custom("unknown typename")),
            }
        } else {
            Err(serde::de::Error::custom("missing __typename field on Node"))
        }
    }
}
