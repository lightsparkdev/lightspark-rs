// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::types::entity_wrapper::EntityWrapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateInvitationWithIncentivesOutput {
    /// The created invitation in the form of a string identifier.
    #[serde(rename = "create_invitation_with_incentives_output_invitation")]
    pub invitation: EntityWrapper,
}

pub const FRAGMENT: &str = "
fragment CreateInvitationWithIncentivesOutputFragment on CreateInvitationWithIncentivesOutput {
    __typename
    create_invitation_with_incentives_output_invitation: invitation {
        id
    }
}
";
