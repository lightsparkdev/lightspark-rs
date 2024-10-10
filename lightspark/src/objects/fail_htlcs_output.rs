
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::types::entity_wrapper::EntityWrapper;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FailHtlcsOutput {

    
    #[serde(rename = "fail_htlcs_output_invoice")]
    pub invoice: EntityWrapper,

}



pub const FRAGMENT: &str = "
fragment FailHtlcsOutputFragment on FailHtlcsOutput {
    __typename
    fail_htlcs_output_invoice: invoice {
        id
    }
}
";



