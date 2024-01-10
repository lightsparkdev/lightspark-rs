
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::types::entity_wrapper::EntityWrapper;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClaimUmaInvitationWithIncentivesOutput {

    
    #[serde(rename = "claim_uma_invitation_with_incentives_output_invitation")]
    pub invitation: EntityWrapper,

}



pub const FRAGMENT: &str = "
fragment ClaimUmaInvitationWithIncentivesOutputFragment on ClaimUmaInvitationWithIncentivesOutput {
    __typename
    claim_uma_invitation_with_incentives_output_invitation: invitation {
        id
    }
}
";



