// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::types::entity_wrapper::EntityWrapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateChannelPerCommitmentPointOutput {
    #[serde(rename = "update_channel_per_commitment_point_output_channel")]
    pub channel: EntityWrapper,
}

pub const FRAGMENT: &str = "
fragment UpdateChannelPerCommitmentPointOutputFragment on UpdateChannelPerCommitmentPointOutput {
    __typename
    update_channel_per_commitment_point_output_channel: channel {
        id
    }
}
";
