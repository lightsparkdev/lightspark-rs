// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This is an enum representing a particular reason why an htlc sent over the Lightning Network may have failed.
#[derive(Clone, Deserialize, Serialize)]
pub enum HtlcAttemptFailureCode {
    #[serde(rename = "INCORRECT_OR_UNKNOWN_PAYMENT_DETAILS")]
    IncorrectOrUnknownPaymentDetails,

    #[serde(rename = "INCORRECT_PAYMENT_AMOUNT")]
    IncorrectPaymentAmount,

    #[serde(rename = "FINAL_INCORRECT_CLTV_EXPIRY")]
    FinalIncorrectCltvExpiry,

    #[serde(rename = "FINAL_INCORRECT_HTLC_AMOUNT")]
    FinalIncorrectHtlcAmount,

    #[serde(rename = "FINAL_EXPIRY_TOO_SOON")]
    FinalExpiryTooSoon,

    #[serde(rename = "INVALID_REALM")]
    InvalidRealm,

    #[serde(rename = "EXPIRY_TOO_SOON")]
    ExpiryTooSoon,

    #[serde(rename = "INVALID_ONION_VERSION")]
    InvalidOnionVersion,

    #[serde(rename = "INVALID_ONION_HMAC")]
    InvalidOnionHmac,

    #[serde(rename = "INVALID_ONION_KEY")]
    InvalidOnionKey,

    #[serde(rename = "AMOUNT_BELOW_MINIMUM")]
    AmountBelowMinimum,

    #[serde(rename = "FEE_INSUFFICIENT")]
    FeeInsufficient,

    #[serde(rename = "INCORRECT_CLTV_EXPIRY")]
    IncorrectCltvExpiry,

    #[serde(rename = "CHANNEL_DISABLED")]
    ChannelDisabled,

    #[serde(rename = "TEMPORARY_CHANNEL_FAILURE")]
    TemporaryChannelFailure,

    #[serde(rename = "REQUIRED_NODE_FEATURE_MISSING")]
    RequiredNodeFeatureMissing,

    #[serde(rename = "REQUIRED_CHANNEL_FEATURE_MISSING")]
    RequiredChannelFeatureMissing,

    #[serde(rename = "UNKNOWN_NEXT_PEER")]
    UnknownNextPeer,

    #[serde(rename = "TEMPORARY_NODE_FAILURE")]
    TemporaryNodeFailure,

    #[serde(rename = "PERMANENT_NODE_FAILURE")]
    PermanentNodeFailure,

    #[serde(rename = "PERMANENT_CHANNEL_FAILURE")]
    PermanentChannelFailure,

    #[serde(rename = "EXPIRY_TOO_FAR")]
    ExpiryTooFar,

    #[serde(rename = "MPP_TIMEOUT")]
    MppTimeout,

    #[serde(rename = "INVALID_ONION_PAYLOAD")]
    InvalidOnionPayload,

    #[serde(rename = "INTERNAL_FAILURE")]
    InternalFailure,

    #[serde(rename = "UNKNOWN_FAILURE")]
    UnknownFailure,

    #[serde(rename = "UNREADABLE_FAILURE")]
    UnreadableFailure,
}

impl From<HtlcAttemptFailureCode> for Value {
    fn from(val: HtlcAttemptFailureCode) -> Self {
        Value::from(val.to_string())
    }
}

impl fmt::Display for HtlcAttemptFailureCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IncorrectOrUnknownPaymentDetails => {
                write!(f, "INCORRECT_OR_UNKNOWN_PAYMENT_DETAILS")
            }
            Self::IncorrectPaymentAmount => write!(f, "INCORRECT_PAYMENT_AMOUNT"),
            Self::FinalIncorrectCltvExpiry => write!(f, "FINAL_INCORRECT_CLTV_EXPIRY"),
            Self::FinalIncorrectHtlcAmount => write!(f, "FINAL_INCORRECT_HTLC_AMOUNT"),
            Self::FinalExpiryTooSoon => write!(f, "FINAL_EXPIRY_TOO_SOON"),
            Self::InvalidRealm => write!(f, "INVALID_REALM"),
            Self::ExpiryTooSoon => write!(f, "EXPIRY_TOO_SOON"),
            Self::InvalidOnionVersion => write!(f, "INVALID_ONION_VERSION"),
            Self::InvalidOnionHmac => write!(f, "INVALID_ONION_HMAC"),
            Self::InvalidOnionKey => write!(f, "INVALID_ONION_KEY"),
            Self::AmountBelowMinimum => write!(f, "AMOUNT_BELOW_MINIMUM"),
            Self::FeeInsufficient => write!(f, "FEE_INSUFFICIENT"),
            Self::IncorrectCltvExpiry => write!(f, "INCORRECT_CLTV_EXPIRY"),
            Self::ChannelDisabled => write!(f, "CHANNEL_DISABLED"),
            Self::TemporaryChannelFailure => write!(f, "TEMPORARY_CHANNEL_FAILURE"),
            Self::RequiredNodeFeatureMissing => write!(f, "REQUIRED_NODE_FEATURE_MISSING"),
            Self::RequiredChannelFeatureMissing => write!(f, "REQUIRED_CHANNEL_FEATURE_MISSING"),
            Self::UnknownNextPeer => write!(f, "UNKNOWN_NEXT_PEER"),
            Self::TemporaryNodeFailure => write!(f, "TEMPORARY_NODE_FAILURE"),
            Self::PermanentNodeFailure => write!(f, "PERMANENT_NODE_FAILURE"),
            Self::PermanentChannelFailure => write!(f, "PERMANENT_CHANNEL_FAILURE"),
            Self::ExpiryTooFar => write!(f, "EXPIRY_TOO_FAR"),
            Self::MppTimeout => write!(f, "MPP_TIMEOUT"),
            Self::InvalidOnionPayload => write!(f, "INVALID_ONION_PAYLOAD"),
            Self::InternalFailure => write!(f, "INTERNAL_FAILURE"),
            Self::UnknownFailure => write!(f, "UNKNOWN_FAILURE"),
            Self::UnreadableFailure => write!(f, "UNREADABLE_FAILURE"),
        }
    }
}
