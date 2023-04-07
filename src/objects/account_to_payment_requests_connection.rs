// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::page_info::PageInfo;
use crate::objects::payment_request::PaymentRequest;
use serde::Deserialize;
use std::vec::Vec;

#[derive(Deserialize)]
pub struct AccountToPaymentRequestsConnection {
    /// The total count of objects in this connection, using the current filters. It is different from the number of objects returned in the current page (in the `entities` field).
    #[serde(rename = "account_to_payment_requests_connection_count")]
    pub count: Option<i64>,

    /// The payment requests for the current page of this connection.
    #[serde(rename = "account_to_payment_requests_connection_entities")]
    pub entities: Vec<Box<dyn PaymentRequest>>,

    /// An object that holds pagination information about the objects in this connection.
    #[serde(rename = "account_to_payment_requests_connection_page_info")]
    pub page_info: PageInfo,
}

pub const FRAGMENT: &str = "
fragment AccountToPaymentRequestsConnectionFragment on AccountToPaymentRequestsConnection {
    __typename
    account_to_payment_requests_connection_count: count
    account_to_payment_requests_connection_entities: entities {
        id
    }
    account_to_payment_requests_connection_page_info: page_info {
        __typename
        page_info_has_next_page: has_next_page
        page_info_has_previous_page: has_previous_page
        page_info_start_cursor: start_cursor
        page_info_end_cursor: end_cursor
    }
}
";
