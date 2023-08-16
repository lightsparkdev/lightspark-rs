// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum of potential purposes set by a user for a Lightspark node.
#[derive(Clone, Deserialize, Serialize)]
pub enum LightsparkNodePurpose {
    #[serde(rename = "SEND")]
    Send,

    #[serde(rename = "RECEIVE")]
    Receive,

    #[serde(rename = "ROUTING")]
    Routing,
}

impl From<LightsparkNodePurpose> for Value {
    fn from(val: LightsparkNodePurpose) -> Self {
        Value::from(val.to_string())
    }
}

impl fmt::Display for LightsparkNodePurpose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Send => write!(f, "SEND"),
            Self::Receive => write!(f, "RECEIVE"),
            Self::Routing => write!(f, "ROUTING"),
        }
    }
}
