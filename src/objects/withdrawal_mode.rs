// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum of the potential modes that your Bitcoin withdrawal can take.
#[derive(Clone, Deserialize, Serialize)]
pub enum WithdrawalMode {
    #[serde(rename = "WALLET_ONLY")]
    WalletOnly,

    #[serde(rename = "WALLET_THEN_CHANNELS")]
    WalletThenChannels,
}

impl From<WithdrawalMode> for Value {
    fn from(val: WithdrawalMode) -> Self {
        Value::from(val.to_string())
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
