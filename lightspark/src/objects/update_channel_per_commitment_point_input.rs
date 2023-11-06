// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateChannelPerCommitmentPointInput {
    pub channel_id: String,

    pub per_commitment_point: String,

    pub per_commitment_point_index: i64,
}
