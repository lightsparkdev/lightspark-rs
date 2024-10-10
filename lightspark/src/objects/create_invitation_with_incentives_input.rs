// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::region_code::RegionCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateInvitationWithIncentivesInput {
    /// The UMA of the user creating the invitation. It will be used to identify the inviter when receiving the invitation.
    pub inviter_uma: String,

    /// The phone hash of the user creating the invitation.
    pub inviter_phone_hash: String,

    /// The region of the user creating the invitation.
    pub inviter_region: RegionCode,
}
