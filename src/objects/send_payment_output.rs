// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::types::entity_wrapper::EntityWrapper;
use serde::Deserialize;

#[derive(Deserialize)]
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
