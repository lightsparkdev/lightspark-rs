// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::types::entity_wrapper::EntityWrapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateUmaInvitationOutput {
    /// The created invitation in the form of a string identifier.
    #[serde(rename = "create_uma_invitation_output_invitation")]
    pub invitation: EntityWrapper,
}

pub const FRAGMENT: &str = "
fragment CreateUmaInvitationOutputFragment on CreateUmaInvitationOutput {
    __typename
    create_uma_invitation_output_invitation: invitation {
        id
    }
}
";
