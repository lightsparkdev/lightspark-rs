// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::types::entity_wrapper::EntityWrapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutgoingPaymentForIdempotencyKeyOutput {
    #[serde(rename = "outgoing_payment_for_idempotency_key_output_payment")]
    pub payment: Option<EntityWrapper>,
}

pub const FRAGMENT: &str = "
fragment OutgoingPaymentForIdempotencyKeyOutputFragment on OutgoingPaymentForIdempotencyKeyOutput {
    __typename
    outgoing_payment_for_idempotency_key_output_payment: payment {
        id
    }
}
";
