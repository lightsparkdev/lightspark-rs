// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum of the potential permissions that a Lightspark user can have in regards to account management.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Permission {
    #[serde(rename = "ALL")]
    All,

    #[serde(rename = "MAINNET_VIEW")]
    MainnetView,

    #[serde(rename = "MAINNET_TRANSACT")]
    MainnetTransact,

    #[serde(rename = "MAINNET_MANAGE")]
    MainnetManage,

    #[serde(rename = "TESTNET_VIEW")]
    TestnetView,

    #[serde(rename = "TESTNET_TRANSACT")]
    TestnetTransact,

    #[serde(rename = "TESTNET_MANAGE")]
    TestnetManage,

    #[serde(rename = "REGTEST_VIEW")]
    RegtestView,

    #[serde(rename = "REGTEST_TRANSACT")]
    RegtestTransact,

    #[serde(rename = "REGTEST_MANAGE")]
    RegtestManage,

    #[serde(rename = "SIGNET_VIEW")]
    SignetView,

    #[serde(rename = "SIGNET_TRANSACT")]
    SignetTransact,

    #[serde(rename = "SIGNET_MANAGE")]
    SignetManage,

    #[serde(rename = "USER_VIEW")]
    UserView,

    #[serde(rename = "USER_MANAGE")]
    UserManage,

    #[serde(rename = "ACCOUNT_VIEW")]
    AccountView,

    #[serde(rename = "ACCOUNT_MANAGE")]
    AccountManage,
}

impl Into<Value> for Permission {
    fn into(self) -> Value {
        Value::from(self.to_string())
    }
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::All => write!(f, "ALL"),
            Self::MainnetView => write!(f, "MAINNET_VIEW"),
            Self::MainnetTransact => write!(f, "MAINNET_TRANSACT"),
            Self::MainnetManage => write!(f, "MAINNET_MANAGE"),
            Self::TestnetView => write!(f, "TESTNET_VIEW"),
            Self::TestnetTransact => write!(f, "TESTNET_TRANSACT"),
            Self::TestnetManage => write!(f, "TESTNET_MANAGE"),
            Self::RegtestView => write!(f, "REGTEST_VIEW"),
            Self::RegtestTransact => write!(f, "REGTEST_TRANSACT"),
            Self::RegtestManage => write!(f, "REGTEST_MANAGE"),
            Self::SignetView => write!(f, "SIGNET_VIEW"),
            Self::SignetTransact => write!(f, "SIGNET_TRANSACT"),
            Self::SignetManage => write!(f, "SIGNET_MANAGE"),
            Self::UserView => write!(f, "USER_VIEW"),
            Self::UserManage => write!(f, "USER_MANAGE"),
            Self::AccountView => write!(f, "ACCOUNT_VIEW"),
            Self::AccountManage => write!(f, "ACCOUNT_MANAGE"),
        }
    }
}
