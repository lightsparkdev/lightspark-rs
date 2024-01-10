
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::types::entity_wrapper::EntityWrapper;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReleaseChannelPerCommitmentSecretOutput {

    /// The channel object after the per-commitment secret release operation.
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



