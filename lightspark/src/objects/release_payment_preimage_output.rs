
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::types::entity_wrapper::EntityWrapper;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReleasePaymentPreimageOutput {

    /// The invoice of the transaction.
    #[serde(rename = "release_payment_preimage_output_invoice")]
    pub invoice: EntityWrapper,

}



pub const FRAGMENT: &str = "
fragment ReleasePaymentPreimageOutputFragment on ReleasePaymentPreimageOutput {
    __typename
    release_payment_preimage_output_invoice: invoice {
        id
    }
}
";



