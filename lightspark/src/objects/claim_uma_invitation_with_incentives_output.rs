// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::types::entity_wrapper::EntityWrapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClaimUmaInvitationWithIncentivesOutput {
    /// An UMA.ME invitation object.
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
