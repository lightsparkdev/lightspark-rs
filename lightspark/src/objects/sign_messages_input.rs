
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::id_and_signature::IdAndSignature;
use std::vec::Vec;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignMessagesInput {

    /// The list of the message ids and signatures.
    
    pub signatures: Vec<IdAndSignature>,

}





