// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct SetInvoicePaymentHashInput {
    /// The invoice that needs to be updated.
    pub invoice_id: String,

    /// The 32-byte hash of the payment preimage.
    pub payment_hash: String,

    /// The 32-byte nonce used to generate the invoice preimage.
    pub preimage_nonce: String,
}
