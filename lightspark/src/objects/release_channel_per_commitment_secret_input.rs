
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReleaseChannelPerCommitmentSecretInput {

    /// The unique identifier of the channel.
    
    pub channel_id: String,

    /// The per-commitment secret to be released.
    
    pub per_commitment_secret: String,

    /// The index associated with the per-commitment secret.
    
    pub per_commitment_index: i64,

}





