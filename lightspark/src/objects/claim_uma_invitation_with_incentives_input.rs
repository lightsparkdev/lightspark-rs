
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::region_code::RegionCode;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClaimUmaInvitationWithIncentivesInput {

    
    
    pub invitation_code: String,

    
    
    pub invitee_uma: String,

    
    
    pub invitee_phone_hash: String,

    
    
    pub invitee_region: RegionCode,

}





