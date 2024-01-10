
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::page_info::PageInfo;
use std::vec::Vec;
use crate::objects::channel_closing_transaction::ChannelClosingTransaction;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WithdrawalRequestToChannelClosingTransactionsConnection {

    /// An object that holds pagination information about the objects in this connection.
    #[serde (rename = "withdrawal_request_to_channel_closing_transactions_connection_page_info")]
    pub page_info: PageInfo,

    /// The total count of objects in this connection, using the current filters. It is different from the number of objects returned in the current page (in the `entities` field).
    #[serde (rename = "withdrawal_request_to_channel_closing_transactions_connection_count")]
    pub count: i64,

    /// The channel closing transactions for the current page of this connection.
    #[serde (rename = "withdrawal_request_to_channel_closing_transactions_connection_entities")]
    pub entities: Vec<ChannelClosingTransaction>,

}



pub const FRAGMENT: &str = "
fragment WithdrawalRequestToChannelClosingTransactionsConnectionFragment on WithdrawalRequestToChannelClosingTransactionsConnection {
    __typename
    withdrawal_request_to_channel_closing_transactions_connection_page_info: page_info {
        __typename
        page_info_has_next_page: has_next_page
        page_info_has_previous_page: has_previous_page
        page_info_start_cursor: start_cursor
        page_info_end_cursor: end_cursor
    }
    withdrawal_request_to_channel_closing_transactions_connection_count: count
    withdrawal_request_to_channel_closing_transactions_connection_entities: entities {
        id
    }
}
";



