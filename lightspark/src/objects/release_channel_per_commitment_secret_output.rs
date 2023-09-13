// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::types::entity_wrapper::EntityWrapper;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct ReleaseChannelPerCommitmentSecretOutput {
    #[serde(rename = "release_channel_per_commitment_secret_output_channel")]
    pub channel: EntityWrapper,
}

pub const FRAGMENT: &str = "
fragment ReleaseChannelPerCommitmentSecretOutputFragment on ReleaseChannelPerCommitmentSecretOutput {
    __typename
    release_channel_per_commitment_secret_output_channel: channel {
        id
    }
}
";
