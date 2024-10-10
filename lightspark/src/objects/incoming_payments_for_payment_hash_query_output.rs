// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::incoming_payment::IncomingPayment;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IncomingPaymentsForPaymentHashQueryOutput {
    #[serde(rename = "incoming_payments_for_payment_hash_query_output_payments")]
    pub payments: Vec<IncomingPayment>,
}

pub const FRAGMENT: &str = "
fragment IncomingPaymentsForPaymentHashQueryOutputFragment on IncomingPaymentsForPaymentHashQueryOutput {
    __typename
    incoming_payments_for_payment_hash_query_output_payments: payments {
        id
    }
}
";
