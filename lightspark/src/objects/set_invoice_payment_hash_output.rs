// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::types::entity_wrapper::EntityWrapper;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct SetInvoicePaymentHashOutput {
    #[serde(rename = "set_invoice_payment_hash_output_invoice")]
    pub invoice: EntityWrapper,
}

pub const FRAGMENT: &str = "
fragment SetInvoicePaymentHashOutputFragment on SetInvoicePaymentHashOutput {
    __typename
    set_invoice_payment_hash_output_invoice: invoice {
        id
    }
}
";
