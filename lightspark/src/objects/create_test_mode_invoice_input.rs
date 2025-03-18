// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::invoice_type::InvoiceType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateTestModeInvoiceInput {
    /// The local node from which to create the invoice.
    pub local_node_id: String,

    /// The amount for which the invoice should be created, in millisatoshis. Setting the amount to 0 will allow the payer to specify an amount.
    pub amount_msats: i64,

    /// An optional memo to include in the invoice.
    pub memo: Option<String>,

    /// The type of invoice to create.
    pub invoice_type: Option<InvoiceType>,
}
