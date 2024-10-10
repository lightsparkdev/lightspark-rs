// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::types::entity_wrapper::EntityWrapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InvoiceForPaymentHashOutput {
    #[serde(rename = "invoice_for_payment_hash_output_invoice")]
    pub invoice: Option<EntityWrapper>,
}

pub const FRAGMENT: &str = "
fragment InvoiceForPaymentHashOutputFragment on InvoiceForPaymentHashOutput {
    __typename
    invoice_for_payment_hash_output_invoice: invoice {
        id
    }
}
";
