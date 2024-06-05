// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::transaction_status::TransactionStatus;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IncomingPaymentsForInvoiceQueryInput {
    pub invoice_id: String,

    /// An optional filter to only query outgoing payments of given statuses.
    pub statuses: Option<Vec<TransactionStatus>>,
}
