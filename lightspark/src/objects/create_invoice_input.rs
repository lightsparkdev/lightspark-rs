// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::invoice_type::InvoiceType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateInvoiceInput {
    /// The node from which to create the invoice.
    pub node_id: String,

    /// The amount for which the invoice should be created, in millisatoshis. Setting the amount to 0 will allow the payer to specify an amount.
    pub amount_msats: i64,

    pub memo: Option<String>,

    pub invoice_type: Option<InvoiceType>,

    /// The expiry of the invoice in seconds. Default value is 86400 (1 day).
    pub expiry_secs: Option<i64>,

    /// The payment hash of the invoice. It should only be set if your node is a remote signing node. If not set, it will be requested through REMOTE_SIGNING webhooks with sub event type REQUEST_INVOICE_PAYMENT_HASH.
    pub payment_hash: Option<String>,

    /// The 32-byte nonce used to generate the invoice preimage if applicable. It will later be included in RELEASE_PAYMENT_PREIMAGE webhook to help recover the raw preimage. This can only be specified when `payment_hash` is specified.
    pub preimage_nonce: Option<String>,
}
