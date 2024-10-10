
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RequestInitiator {

    #[serde(rename="CUSTOMER")]
    Customer,

    #[serde(rename="LIGHTSPARK")]
    Lightspark,

}

impl Into<Value> for RequestInitiator {
    fn into(self) -> Value {
        Value::from(self.to_string())
    }
}

impl fmt::Display for RequestInitiator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Customer => write!(f, "CUSTOMER"),
            Self::Lightspark => write!(f, "LIGHTSPARK"),

        }
    }
}

