
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClaimUmaInvitationInput {

    
    
    pub invitation_code: String,

    
    
    pub invitee_uma: String,

}





