// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct ReleaseChannelPerCommitmentSecretInput {
    pub channel_id: String,

    pub per_commitment_secret: String,

    pub per_commitment_index: i64,
}
