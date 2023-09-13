// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::compliance_provider::ComplianceProvider;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct ScreenNodeInput {
    pub provider: ComplianceProvider,

    pub node_pubkey: String,
}
