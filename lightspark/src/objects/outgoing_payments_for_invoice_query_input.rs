
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use crate::objects::transaction_status::TransactionStatus;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutgoingPaymentsForInvoiceQueryInput {

    /// The encoded invoice that the outgoing payments paid to.
    
    pub encoded_invoice: String,

    /// An optional filter to only query outgoing payments of given statuses.
    
    pub statuses: Option<Vec<TransactionStatus>>,

}





