
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FailHtlcsInput {

    /// The id of invoice which the pending HTLCs that need to be failed are paying for.
    
    pub invoice_id: String,

    /// Whether the invoice needs to be canceled after failing the htlcs. If yes, the invoice cannot be paid anymore.
    
    pub cancel_invoice: bool,

}





