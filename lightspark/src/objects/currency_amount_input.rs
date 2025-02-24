// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::currency_unit::CurrencyUnit;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CurrencyAmountInput {
    pub value: i64,

    pub unit: CurrencyUnit,
}
