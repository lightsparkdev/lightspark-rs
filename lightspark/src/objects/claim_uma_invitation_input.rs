// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClaimUmaInvitationInput {
    /// The unique code that identifies this invitation and was shared by the inviter.
    pub invitation_code: String,

    /// The UMA of the user claiming the invitation. It will be sent to the inviter so that they can start transacting with the invitee.
    pub invitee_uma: String,
}
