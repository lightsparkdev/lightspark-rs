
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::transaction_status::TransactionStatus;
use std::vec::Vec;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IncomingPaymentsForPaymentHashQueryInput {

    /// The 32-byte hash of the payment preimage for which to fetch payments
    
    pub payment_hash: String,

    /// An optional filter to only query incoming payments of given statuses.
    
    pub statuses: Option<Vec<TransactionStatus>>,

}





