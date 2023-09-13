// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KycStatus {
    #[serde(rename = "UNKNOWN")]
    KycStatusUnknown,

    #[serde(rename = "NOT_VERIFIED")]
    KycStatusNotVerified,

    #[serde(rename = "PENDING")]
    KycStatusPending,

    #[serde(rename = "VERIFIED")]
    KycStatusVerified,
}
