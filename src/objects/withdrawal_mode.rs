// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Clone, Deserialize, Serialize)]
pub enum WithdrawalMode {
    #[serde(rename = "WALLET_ONLY")]
    WalletOnly,

    #[serde(rename = "WALLET_THEN_CHANNELS")]
    WalletThenChannels,
}

impl Into<Value> for WithdrawalMode {
    fn into(self) -> Value {
        Value::from(self.to_string())
    }
}

impl fmt::Display for WithdrawalMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::WalletOnly => write!(f, "WALLET_ONLY"),
            Self::WalletThenChannels => write!(f, "WALLET_THEN_CHANNELS"),
        }
    }
}
