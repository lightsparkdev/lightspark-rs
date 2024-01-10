
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use crate::objects::signable_payload::SignablePayload;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignMessagesOutput {

    /// The list of signed payloads.
    #[serde (rename = "sign_messages_output_signed_payloads")]
    pub signed_payloads: Vec<SignablePayload>,

}



pub const FRAGMENT: &str = "
fragment SignMessagesOutputFragment on SignMessagesOutput {
    __typename
    sign_messages_output_signed_payloads: signed_payloads {
        id
    }
}
";



