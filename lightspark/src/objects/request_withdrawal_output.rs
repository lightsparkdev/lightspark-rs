
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::types::entity_wrapper::EntityWrapper;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestWithdrawalOutput {

    /// The request that is created for this withdrawal.
    #[serde(rename = "request_withdrawal_output_request")]
    pub request: EntityWrapper,

}



pub const FRAGMENT: &str = "
fragment RequestWithdrawalOutputFragment on RequestWithdrawalOutput {
    __typename
    request_withdrawal_output_request: request {
        id
    }
}
";



