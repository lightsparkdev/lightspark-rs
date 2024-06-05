// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PayUmaInvoiceInput {
    pub node_id: String,

    pub encoded_invoice: String,

    pub timeout_secs: i64,

    pub maximum_fees_msats: i64,

    pub amount_msats: Option<i64>,

    pub idempotency_key: Option<String>,
}
