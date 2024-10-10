
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};


/// The unique identifier of the Invoice that should be cancelled. The invoice is supposed to be open, not settled and not expired.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CancelInvoiceInput {

    
    
    pub invoice_id: String,

}





