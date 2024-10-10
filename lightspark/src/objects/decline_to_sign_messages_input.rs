
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use std::vec::Vec;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeclineToSignMessagesInput {

    /// List of payload ids to decline to sign because validation failed.
    
    pub payload_ids: Vec<String>,

}





