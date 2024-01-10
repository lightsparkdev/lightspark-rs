
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateLnurlInvoiceInput {

    /// The node from which to create the invoice.
    
    pub node_id: String,

    /// The amount for which the invoice should be created, in millisatoshis.
    
    pub amount_msats: i64,

    /// The SHA256 hash of the LNURL metadata payload. This will be present in the h-tag (SHA256 purpose of payment) of the resulting Bolt 11 invoice.
    
    pub metadata_hash: String,

    /// The expiry of the invoice in seconds. Default value is 86400 (1 day).
    
    pub expiry_secs: Option<i64>,

}





