// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Clone, Deserialize, Serialize)]
pub enum BitcoinNetwork {
    /// The production version of the Bitcoin Blockchain.

    #[serde(rename = "MAINNET")]
    Mainnet,
    /// A test version of the Bitcoin Blockchain, maintained by Lightspark.

    #[serde(rename = "REGTEST")]
    Regtest,
    /// A test version of the Bitcoin Blockchain, maintained by a centralized organization. Not in use at Lightspark.

    #[serde(rename = "SIGNET")]
    Signet,
    /// A test version of the Bitcoin Blockchain, publically available.

    #[serde(rename = "TESTNET")]
    Testnet,
}

impl Into<Value> for BitcoinNetwork {
    fn into(self) -> Value {
        Value::from(self.to_string())
    }
}

impl fmt::Display for BitcoinNetwork {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Mainnet => write!(f, "MAINNET"),
            Self::Regtest => write!(f, "REGTEST"),
            Self::Signet => write!(f, "SIGNET"),
            Self::Testnet => write!(f, "TESTNET"),
        }
    }
}
