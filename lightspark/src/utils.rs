use crate::{error::Error, objects::currency_amount::CurrencyAmount};

pub fn value_millisatoshi(amount: &CurrencyAmount) -> Result<i64, Error> {
    match amount.original_unit {
        crate::objects::currency_unit::CurrencyUnit::Bitcoin => {
            Ok(amount.original_value * 100_000_000_000)
        }
        crate::objects::currency_unit::CurrencyUnit::Satoshi => Ok(amount.original_value * 1000),
        crate::objects::currency_unit::CurrencyUnit::Millisatoshi => Ok(amount.original_value),
        crate::objects::currency_unit::CurrencyUnit::Usd => Err(Error::InvalidCurrencyConversion),
        crate::objects::currency_unit::CurrencyUnit::Nanobitcoin => Ok(amount.original_value * 100),
        crate::objects::currency_unit::CurrencyUnit::Microbitcoin => {
            Ok(amount.original_value * 100_000)
        }
        crate::objects::currency_unit::CurrencyUnit::Millibitcoin => {
            Ok(amount.original_value * 100_000_000)
        }
    }
}
