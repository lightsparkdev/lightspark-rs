
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use crate::objects::payment_failure_reason::PaymentFailureReason;
use crate::objects::routing_transaction_failure_reason::RoutingTransactionFailureReason;

/// This object represents payment failures associated with your Lightspark Node.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransactionFailures {

    
    
    pub payment_failures: Option<Vec<PaymentFailureReason>>,

    
    
    pub routing_transaction_failures: Option<Vec<RoutingTransactionFailureReason>>,

}





