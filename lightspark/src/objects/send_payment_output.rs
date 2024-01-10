
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::types::entity_wrapper::EntityWrapper;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendPaymentOutput {

    /// The payment that has been sent.
    #[serde(rename = "send_payment_output_payment")]
    pub payment: EntityWrapper,

}



pub const FRAGMENT: &str = "
fragment SendPaymentOutputFragment on SendPaymentOutput {
    __typename
    send_payment_output_payment: payment {
        id
    }
}
";



