
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::currency_amount::CurrencyAmount;
use std::vec::Vec;
use crate::objects::connection::Connection;
use crate::objects::transaction::Transaction;
use crate::objects::transaction::TransactionEnum;
use crate::objects::page_info::PageInfo;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountToTransactionsConnection {

    /// The total count of objects in this connection, using the current filters. It is different from the number of objects returned in the current page (in the `entities` field).
    #[serde (rename = "account_to_transactions_connection_count")]
    pub count: i64,

    /// An object that holds pagination information about the objects in this connection.
    #[serde (rename = "account_to_transactions_connection_page_info")]
    pub page_info: PageInfo,

    /// Profit (or loss) generated by the transactions in this connection, with the set of filters and constraints provided.
    #[serde (rename = "account_to_transactions_connection_profit_loss")]
    pub profit_loss: Option<CurrencyAmount>,

    /// Average fee earned for the transactions in this connection, with the set of filters and constraints provided.
    #[serde (rename = "account_to_transactions_connection_average_fee_earned")]
    pub average_fee_earned: Option<CurrencyAmount>,

    /// Total amount transacted by the transactions in this connection, with the set of filters and constraints provided.
    #[serde (rename = "account_to_transactions_connection_total_amount_transacted")]
    pub total_amount_transacted: Option<CurrencyAmount>,

    /// The transactions for the current page of this connection.
    #[serde (rename = "account_to_transactions_connection_entities")]
    pub entities: Vec<TransactionEnum>,

    /// The typename of the object
    #[serde(rename = "__typename")]
    pub typename: String,

}


impl Connection for AccountToTransactionsConnection {

    /// The total count of objects in this connection, using the current filters. It is different from the number of objects returned in the current page (in the `entities` field).
    fn get_count(&self) -> i64 {
        self.count
    }

    /// An object that holds pagination information about the objects in this connection.
    fn get_page_info(&self) -> PageInfo {
        self.page_info.clone()
    }


    fn type_name(&self) -> &'static str {
        "AccountToTransactionsConnection"
    }
}




pub const FRAGMENT: &str = "
fragment AccountToTransactionsConnectionFragment on AccountToTransactionsConnection {
    __typename
    account_to_transactions_connection_count: count
    account_to_transactions_connection_page_info: page_info {
        __typename
        page_info_has_next_page: has_next_page
        page_info_has_previous_page: has_previous_page
        page_info_start_cursor: start_cursor
        page_info_end_cursor: end_cursor
    }
    account_to_transactions_connection_profit_loss: profit_loss {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    account_to_transactions_connection_average_fee_earned: average_fee_earned {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    account_to_transactions_connection_total_amount_transacted: total_amount_transacted {
        __typename
        currency_amount_original_value: original_value
        currency_amount_original_unit: original_unit
        currency_amount_preferred_currency_unit: preferred_currency_unit
        currency_amount_preferred_currency_value_rounded: preferred_currency_value_rounded
        currency_amount_preferred_currency_value_approx: preferred_currency_value_approx
    }
    account_to_transactions_connection_entities: entities {
        id
    }
}
";



