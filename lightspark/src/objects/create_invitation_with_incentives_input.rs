
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::region_code::RegionCode;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateInvitationWithIncentivesInput {

    
    
    pub inviter_uma: String,

    
    
    pub inviter_phone_hash: String,

    
    
    pub inviter_region: RegionCode,

}





