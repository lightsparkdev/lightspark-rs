
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::currency_amount::CurrencyAmount;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChannelToTransactionsConnection {

    /// The total count of objects in this connection, using the current filters. It is different from the number of objects returned in the current page (in the `entities` field).
    #[serde (rename = "channel_to_transactions_connection_count")]
    pub count: i64,

    /// The average fee for the transactions that transited through this channel, according to the filters and constraints of the connection.
    #[serde (rename = "channel_to_transactions_connection_average_fee")]
    pub average_fee: Option<CurrencyAmount>,

    /// The total amount transacted for the transactions that transited through this channel, according to the filters and constraints of the connection.
    #[serde (rename = "channel_to_transactions_connection_total_amount_transacted")]
    pub total_amount_transacted: Option<CurrencyAmount>,

    /// The total amount of fees for the transactions that transited through this channel, according to the filters and constraints of the connection.
    #[serde (rename = "channel_to_transactions_connection_total_fees")]
    pub total_fees: Option<CurrencyAmount>,

}



pub const FRAGMENT: &str = "
fragment ChannelToTransactionsConnectionFragment on ChannelToTransactionsConnection {
    __typename
    channel_to_transactions_connection_count: count
    channel_to_transactions_connection_average_fee: average_fee {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    channel_to_transactions_connection_total_amount_transacted: total_amount_transacted {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    channel_to_transactions_connection_total_fees: total_fees {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
}
";



