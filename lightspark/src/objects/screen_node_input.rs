
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::compliance_provider::ComplianceProvider;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScreenNodeInput {

    /// The compliance provider that is going to screen the node. You need to be a customer of the selected provider and store the API key on the Lightspark account setting page.
    
    pub provider: ComplianceProvider,

    /// The public key of the lightning node that needs to be screened.
    
    pub node_pubkey: String,

}





