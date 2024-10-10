// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::signable_payload::SignablePayload;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeclineToSignMessagesOutput {
    #[serde(rename = "decline_to_sign_messages_output_declined_payloads")]
    pub declined_payloads: Vec<SignablePayload>,
}

pub const FRAGMENT: &str = "
fragment DeclineToSignMessagesOutputFragment on DeclineToSignMessagesOutput {
    __typename
    decline_to_sign_messages_output_declined_payloads: declined_payloads {
        id
    }
}
";
