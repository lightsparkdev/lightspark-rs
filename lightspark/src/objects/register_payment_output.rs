// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::types::entity_wrapper::EntityWrapper;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct RegisterPaymentOutput {
    #[serde(rename = "register_payment_output_payment")]
    pub payment: EntityWrapper,
}

pub const FRAGMENT: &str = "
fragment RegisterPaymentOutputFragment on RegisterPaymentOutput {
    __typename
    register_payment_output_payment: payment {
        id
    }
}
";
