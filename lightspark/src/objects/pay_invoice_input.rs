
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PayInvoiceInput {

    /// The node from where you want to send the payment.
    
    pub node_id: String,

    /// The invoice you want to pay (as defined by the BOLT11 standard).
    
    pub encoded_invoice: String,

    /// The timeout in seconds that we will try to make the payment.
    
    pub timeout_secs: i64,

    /// The maximum amount of fees that you want to pay for this payment to be sent, expressed in msats.
    
    pub maximum_fees_msats: i64,

    /// The amount you will pay for this invoice, expressed in msats. It should ONLY be set when the invoice amount is zero.
    
    pub amount_msats: Option<i64>,

}





