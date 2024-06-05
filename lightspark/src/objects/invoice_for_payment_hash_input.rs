// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InvoiceForPaymentHashInput {
    /// The 32-byte hash of the payment preimage for which to fetch an invoice.
    pub payment_hash: String,
}
