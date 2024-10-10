// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::outgoing_payment::OutgoingPayment;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutgoingPaymentsForPaymentHashQueryOutput {
    #[serde(rename = "outgoing_payments_for_payment_hash_query_output_payments")]
    pub payments: Vec<OutgoingPayment>,
}

pub const FRAGMENT: &str = "
fragment OutgoingPaymentsForPaymentHashQueryOutputFragment on OutgoingPaymentsForPaymentHashQueryOutput {
    __typename
    outgoing_payments_for_payment_hash_query_output_payments: payments {
        id
    }
}
";
