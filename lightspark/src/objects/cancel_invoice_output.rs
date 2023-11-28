// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::types::entity_wrapper::EntityWrapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CancelInvoiceOutput {
    #[serde(rename = "cancel_invoice_output_invoice")]
    pub invoice: EntityWrapper,
}

pub const FRAGMENT: &str = "
fragment CancelInvoiceOutputFragment on CancelInvoiceOutput {
    __typename
    cancel_invoice_output_invoice: invoice {
        id
    }
}
";
