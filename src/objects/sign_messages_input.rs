// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::id_and_signature::IdAndSignature;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Clone, Deserialize, Serialize)]
pub struct SignMessagesInput {
    pub signatures: Vec<IdAndSignature>,
}
