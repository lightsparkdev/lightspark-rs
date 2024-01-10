
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::currency_amount::CurrencyAmount;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FundNodeOutput {

    
    #[serde (rename = "fund_node_output_amount")]
    pub amount: CurrencyAmount,

}



pub const FRAGMENT: &str = "
fragment FundNodeOutputFragment on FundNodeOutput {
    __typename
    fund_node_output_amount: amount {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
}
";



