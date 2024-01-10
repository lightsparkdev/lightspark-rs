
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::types::entity_wrapper::EntityWrapper;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClaimUmaInvitationOutput {

    
    #[serde(rename = "claim_uma_invitation_output_invitation")]
    pub invitation: EntityWrapper,

}



pub const FRAGMENT: &str = "
fragment ClaimUmaInvitationOutputFragment on ClaimUmaInvitationOutput {
    __typename
    claim_uma_invitation_output_invitation: invitation {
        id
    }
}
";



