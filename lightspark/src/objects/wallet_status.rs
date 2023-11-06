// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum of the potential statuses that your Lightspark wallet can take.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum WalletStatus {
    /// The wallet has not been set up yet and is ready to be deployed. This is the default status after the first login.

    #[serde(rename = "NOT_SETUP")]
    NotSetup,
    /// The wallet is currently being deployed in the Lightspark infrastructure.

    #[serde(rename = "DEPLOYING")]
    Deploying,
    /// The wallet has been deployed in the Lightspark infrastructure and is ready to be initialized.

    #[serde(rename = "DEPLOYED")]
    Deployed,
    /// The wallet is currently being initialized.

    #[serde(rename = "INITIALIZING")]
    Initializing,
    /// The wallet is available and ready to be used.

    #[serde(rename = "READY")]
    Ready,
    /// The wallet is temporarily available, due to a transient issue or a scheduled maintenance.

    #[serde(rename = "UNAVAILABLE")]
    Unavailable,
    /// The wallet had an unrecoverable failure. This status is not expected to happend and will be investigated by the Lightspark team.

    #[serde(rename = "FAILED")]
    Failed,
    /// The wallet is being terminated.

    #[serde(rename = "TERMINATING")]
    Terminating,
    /// The wallet has been terminated and is not available in the Lightspark infrastructure anymore. It is not connected to the Lightning network and its funds can only be accessed using the Funds Recovery flow.

    #[serde(rename = "TERMINATED")]
    Terminated,
}

impl From<WalletStatus> for Value {
    fn from(val: WalletStatus) -> Self {
        Value::from(val.to_string())
    }
}

impl fmt::Display for WalletStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotSetup => write!(f, "NOT_SETUP"),
            Self::Deploying => write!(f, "DEPLOYING"),
            Self::Deployed => write!(f, "DEPLOYED"),
            Self::Initializing => write!(f, "INITIALIZING"),
            Self::Ready => write!(f, "READY"),
            Self::Unavailable => write!(f, "UNAVAILABLE"),
            Self::Failed => write!(f, "FAILED"),
            Self::Terminating => write!(f, "TERMINATING"),
            Self::Terminated => write!(f, "TERMINATED"),
        }
    }
}
