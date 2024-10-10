
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::hop::Hop;
use std::vec::Vec;
use crate::objects::page_info::PageInfo;
use crate::objects::connection::Connection;

/// The connection from an outgoing payment attempt to the list of sequential hops that define the path from sender node to recipient node.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutgoingPaymentAttemptToHopsConnection {

    /// The total count of objects in this connection, using the current filters. It is different from the number of objects returned in the current page (in the `entities` field).
    #[serde (rename = "outgoing_payment_attempt_to_hops_connection_count")]
    pub count: i64,

    /// An object that holds pagination information about the objects in this connection.
    #[serde (rename = "outgoing_payment_attempt_to_hops_connection_page_info")]
    pub page_info: PageInfo,

    /// The hops for the current page of this connection.
    #[serde (rename = "outgoing_payment_attempt_to_hops_connection_entities")]
    pub entities: Vec<Hop>,

    /// The typename of the object
    #[serde(rename = "__typename")]
    pub typename: String,

}


impl Connection for OutgoingPaymentAttemptToHopsConnection {

    /// The total count of objects in this connection, using the current filters. It is different from the number of objects returned in the current page (in the `entities` field).
    fn get_count(&self) -> i64 {
        self.count
    }

    /// An object that holds pagination information about the objects in this connection.
    fn get_page_info(&self) -> PageInfo {
        self.page_info.clone()
    }


    fn type_name(&self) -> &'static str {
        "OutgoingPaymentAttemptToHopsConnection"
    }
}




pub const FRAGMENT: &str = "
fragment OutgoingPaymentAttemptToHopsConnectionFragment on OutgoingPaymentAttemptToHopsConnection {
    __typename
    outgoing_payment_attempt_to_hops_connection_count: count
    outgoing_payment_attempt_to_hops_connection_page_info: page_info {
        __typename
        page_info_has_next_page: has_next_page
        page_info_has_previous_page: has_previous_page
        page_info_start_cursor: start_cursor
        page_info_end_cursor: end_cursor
    }
    outgoing_payment_attempt_to_hops_connection_entities: entities {
        id
    }
}
";



