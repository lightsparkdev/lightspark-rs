// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct ReleasePaymentPreimageInput {
    /// The invoice the preimage belongs to.
    pub invoice_id: String,

    /// The preimage to release.
    pub payment_preimage: String,
}
