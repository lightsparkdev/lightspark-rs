// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutgoingPaymentForIdempotencyKeyInput {
    pub idempotency_key: String,
}
