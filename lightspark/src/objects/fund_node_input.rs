
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FundNodeInput {

    
    
    pub node_id: String,

    
    
    pub amount_sats: Option<i64>,

    
    
    pub funding_address: Option<String>,

}





