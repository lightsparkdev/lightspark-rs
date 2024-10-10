
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateNodeSharedSecretInput {

    
    
    pub node_id: String,

    
    
    pub shared_secret: String,

}





