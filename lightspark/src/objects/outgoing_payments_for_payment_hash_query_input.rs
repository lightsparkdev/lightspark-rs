// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::transaction_status::TransactionStatus;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutgoingPaymentsForPaymentHashQueryInput {
    /// The 32-byte hash of the payment preimage for which to fetch payments
    pub payment_hash: String,

    /// An optional filter to only query outgoing payments of given statuses.
    pub statuses: Option<Vec<TransactionStatus>>,
}
