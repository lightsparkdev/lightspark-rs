
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::invoice_type::InvoiceType;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateTestModeInvoiceInput {

    
    
    pub local_node_id: String,

    
    
    pub amount_msats: i64,

    
    
    pub memo: Option<String>,

    
    
    pub invoice_type: Option<InvoiceType>,

}





