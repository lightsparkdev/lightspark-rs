// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::types::entity_wrapper::EntityWrapper;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct UpdateNodeSharedSecretOutput {
    #[serde(rename = "update_node_shared_secret_output_node")]
    pub node: EntityWrapper,
}

pub const FRAGMENT: &str = "
fragment UpdateNodeSharedSecretOutputFragment on UpdateNodeSharedSecretOutput {
    __typename
    update_node_shared_secret_output_node: node {
        id
    }
}
";
