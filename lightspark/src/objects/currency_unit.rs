
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// This enum identifies the unit of currency associated with a CurrencyAmount.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CurrencyUnit {
    /// Bitcoin is the cryptocurrency native to the Bitcoin network. It is used as the native medium for value transfer for the Lightning Network.

    #[serde(rename="BITCOIN")]
    Bitcoin,
    /// 0.00000001 (10e-8) Bitcoin or one hundred millionth of a Bitcoin. This is the unit most commonly used in Lightning transactions.

    #[serde(rename="SATOSHI")]
    Satoshi,
    /// 0.001 Satoshi, or 10e-11 Bitcoin. We recommend using the Satoshi unit instead when possible.

    #[serde(rename="MILLISATOSHI")]
    Millisatoshi,
    /// United States Dollar.

    #[serde(rename="USD")]
    Usd,
    /// Mexican Peso.

    #[serde(rename="MXN")]
    Mxn,
    /// 0.000000001 (10e-9) Bitcoin or a billionth of a Bitcoin. We recommend using the Satoshi unit instead when possible.

    #[serde(rename="NANOBITCOIN")]
    Nanobitcoin,
    /// 0.000001 (10e-6) Bitcoin or a millionth of a Bitcoin. We recommend using the Satoshi unit instead when possible.

    #[serde(rename="MICROBITCOIN")]
    Microbitcoin,
    /// 0.001 (10e-3) Bitcoin or a thousandth of a Bitcoin. We recommend using the Satoshi unit instead when possible.

    #[serde(rename="MILLIBITCOIN")]
    Millibitcoin,

}

impl Into<Value> for CurrencyUnit {
    fn into(self) -> Value {
        Value::from(self.to_string())
    }
}

impl fmt::Display for CurrencyUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bitcoin => write!(f, "BITCOIN"),
            Self::Satoshi => write!(f, "SATOSHI"),
            Self::Millisatoshi => write!(f, "MILLISATOSHI"),
            Self::Usd => write!(f, "USD"),
            Self::Mxn => write!(f, "MXN"),
            Self::Nanobitcoin => write!(f, "NANOBITCOIN"),
            Self::Microbitcoin => write!(f, "MICROBITCOIN"),
            Self::Millibitcoin => write!(f, "MILLIBITCOIN"),

        }
    }
}

