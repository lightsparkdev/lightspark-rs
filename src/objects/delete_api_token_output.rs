// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::types::entity_wrapper::EntityWrapper;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DeleteApiTokenOutput {
    #[serde(rename = "delete_api_token_output_account")]
    pub account: EntityWrapper,
}

pub const FRAGMENT: &str = "
fragment DeleteApiTokenOutputFragment on DeleteApiTokenOutput {
    __typename
    delete_api_token_output_account: account {
        id
    }
}
";
