// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::outgoing_payment::OutgoingPayment;
use serde::Deserialize;
use std::vec::Vec;

#[derive(Clone, Deserialize)]
pub struct OutgoingPaymentsForInvoiceQueryOutput {
    #[serde(rename = "outgoing_payments_for_invoice_query_output_payments")]
    pub payments: Vec<OutgoingPayment>,
}

pub const FRAGMENT: &str = "
fragment OutgoingPaymentsForInvoiceQueryOutputFragment on OutgoingPaymentsForInvoiceQueryOutput {
    __typename
    outgoing_payments_for_invoice_query_output_payments: payments {
        id
    }
}
";
