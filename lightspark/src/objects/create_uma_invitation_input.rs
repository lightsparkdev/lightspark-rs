// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateUmaInvitationInput {
    /// The UMA of the user creating the invitation. It will be used to identify the inviter when receiving the invitation.
    pub inviter_uma: String,
}
