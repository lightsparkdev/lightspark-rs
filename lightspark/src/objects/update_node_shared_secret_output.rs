
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::types::entity_wrapper::EntityWrapper;


#[derive(Debug, Clone, Deserialize, Serialize)]
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



