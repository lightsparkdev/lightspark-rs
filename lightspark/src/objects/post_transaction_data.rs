
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::currency_amount::CurrencyAmount;

/// This object represents post-transaction data that could be used to register payment for KYT.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostTransactionData {

    /// The utxo of the channel over which the payment went through in the format of <transaction_hash>:<output_index>.
    #[serde (rename = "post_transaction_data_utxo")]
    pub utxo: String,

    /// The amount of funds transferred in the payment.
    #[serde (rename = "post_transaction_data_amount")]
    pub amount: CurrencyAmount,

}



pub const FRAGMENT: &str = "
fragment PostTransactionDataFragment on PostTransactionData {
    __typename
    post_transaction_data_utxo: utxo
    post_transaction_data_amount: amount {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
}
";



