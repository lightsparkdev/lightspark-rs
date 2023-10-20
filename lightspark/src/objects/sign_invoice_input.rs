// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct SignInvoiceInput {
    /// The unique identifier of the invoice to be signed.
    pub invoice_id: String,

    /// The cryptographic signature for the invoice.
    pub signature: String,

    /// The recovery identifier for the signature.
    pub recovery_id: i64,
}
