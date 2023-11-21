// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::region_code::RegionCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateInvitationWithIncentivesInput {
    pub inviter_uma: String,

    pub inviter_phone_hash: String,

    pub inviter_region: RegionCode,
}
