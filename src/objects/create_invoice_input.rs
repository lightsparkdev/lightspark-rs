// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::invoice_type::InvoiceType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct CreateInvoiceInput {
    /// The node from which to create the invoice.
    pub node_id: String,

    /// The amount for which the invoice should be created, in millisatoshis.
    pub amount_msats: i64,

    pub memo: Option<String>,

    pub invoice_type: Option<InvoiceType>,

    /// The expiry of the invoice in seconds. Default value is 86400 (1 day).
    pub expiry_secs: Option<i64>,

    pub payment_hash: Option<String>,

    pub preimage_nonce: Option<String>,
}
