// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Clone, Deserialize, Serialize)]
pub enum LightsparkNodeStatus {
    #[serde(rename = "CREATED")]
    Created,

    #[serde(rename = "DEPLOYED")]
    Deployed,

    #[serde(rename = "STARTED")]
    Started,

    #[serde(rename = "SYNCING")]
    Syncing,

    #[serde(rename = "READY")]
    Ready,

    #[serde(rename = "STOPPED")]
    Stopped,

    #[serde(rename = "TERMINATED")]
    Terminated,

    #[serde(rename = "TERMINATING")]
    Terminating,

    #[serde(rename = "WALLET_LOCKED")]
    WalletLocked,

    #[serde(rename = "FAILED_TO_DEPLOY")]
    FailedToDeploy,
}

impl From<LightsparkNodeStatus> for Value {
    fn from(val: LightsparkNodeStatus) -> Self {
        Value::from(val.to_string())
    }
}

impl fmt::Display for LightsparkNodeStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Created => write!(f, "CREATED"),
            Self::Deployed => write!(f, "DEPLOYED"),
            Self::Started => write!(f, "STARTED"),
            Self::Syncing => write!(f, "SYNCING"),
            Self::Ready => write!(f, "READY"),
            Self::Stopped => write!(f, "STOPPED"),
            Self::Terminated => write!(f, "TERMINATED"),
            Self::Terminating => write!(f, "TERMINATING"),
            Self::WalletLocked => write!(f, "WALLET_LOCKED"),
            Self::FailedToDeploy => write!(f, "FAILED_TO_DEPLOY"),
        }
    }
}
