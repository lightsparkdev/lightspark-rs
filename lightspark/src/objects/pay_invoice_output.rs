// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::types::entity_wrapper::EntityWrapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PayInvoiceOutput {
    /// The payment that has been sent.
    #[serde(rename = "pay_invoice_output_payment")]
    pub payment: EntityWrapper,
}

pub const FRAGMENT: &str = "
fragment PayInvoiceOutputFragment on PayInvoiceOutput {
    __typename
    pay_invoice_output_payment: payment {
        id
    }
}
";
