// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum of the potential types of addresses that a node on the Lightning Network can have.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum NodeAddressType {
    #[serde(rename = "IPV4")]
    Ipv4,

    #[serde(rename = "IPV6")]
    Ipv6,

    #[serde(rename = "TOR")]
    Tor,
}

impl From<NodeAddressType> for Value {
    fn from(val: NodeAddressType) -> Self {
        Value::from(val.to_string())
    }
}

impl fmt::Display for NodeAddressType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Ipv4 => write!(f, "IPV4"),
            Self::Ipv6 => write!(f, "IPV6"),
            Self::Tor => write!(f, "TOR"),
        }
    }
}
