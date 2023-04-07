// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// An enum that enumerates all possible types of addresses of a node on the Lightning Network.
#[derive(Clone, Deserialize, Serialize)]
pub enum NodeAddressType {
    #[serde(rename = "IPV4")]
    Ipv4,

    #[serde(rename = "IPV6")]
    Ipv6,

    #[serde(rename = "TOR")]
    Tor,
}

impl Into<Value> for NodeAddressType {
    fn into(self) -> Value {
        Value::from(self.to_string())
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
