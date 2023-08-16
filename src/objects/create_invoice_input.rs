// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::invoice_type::InvoiceType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct CreateInvoiceInput {
    pub node_id: String,

    pub amount_msats: i64,

    pub memo: Option<String>,

    pub invoice_type: Option<InvoiceType>,

    pub payment_hash: Option<String>,

    pub preimage_nonce: Option<String>,
}
