// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub symbol: String,
    #[serde(rename = "multiplier")]
    pub millisatoshi_per_unit: i64,
    #[serde(rename = "minSendable")]
    pub min_sendable: i64,
    #[serde(rename = "maxSendable")]
    pub max_sendable: i64,
}
