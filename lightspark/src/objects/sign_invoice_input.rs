// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct SignInvoiceInput {
    pub invoice_id: String,

    pub signature: String,

    pub recovery_id: i64,
}
