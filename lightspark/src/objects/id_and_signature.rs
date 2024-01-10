
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IdAndSignature {

    /// The id of the message.
    
    pub id: String,

    /// The signature of the message.
    
    pub signature: String,

}





