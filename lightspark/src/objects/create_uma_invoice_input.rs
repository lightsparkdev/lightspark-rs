// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateUmaInvoiceInput {
    pub node_id: String,

    pub amount_msats: i64,

    pub metadata_hash: String,

    pub expiry_secs: Option<i64>,
}
