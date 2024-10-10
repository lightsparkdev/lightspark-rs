// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::channel::Channel;
use crate::objects::connection::Connection;
use crate::objects::page_info::PageInfo;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountToChannelsConnection {
    /// The total count of objects in this connection, using the current filters. It is different from the number of objects returned in the current page (in the `entities` field).
    #[serde(rename = "account_to_channels_connection_count")]
    pub count: i64,

    /// An object that holds pagination information about the objects in this connection.
    #[serde(rename = "account_to_channels_connection_page_info")]
    pub page_info: PageInfo,

    /// The channels for the current page of this connection.
    #[serde(rename = "account_to_channels_connection_entities")]
    pub entities: Vec<Channel>,

    /// The typename of the object
    #[serde(rename = "__typename")]
    pub typename: String,
}

impl Connection for AccountToChannelsConnection {
    /// The total count of objects in this connection, using the current filters. It is different from the number of objects returned in the current page (in the `entities` field).
    fn get_count(&self) -> i64 {
        self.count
    }

    /// An object that holds pagination information about the objects in this connection.
    fn get_page_info(&self) -> PageInfo {
        self.page_info.clone()
    }

    fn type_name(&self) -> &'static str {
        "AccountToChannelsConnection"
    }
}

pub const FRAGMENT: &str = "
fragment AccountToChannelsConnectionFragment on AccountToChannelsConnection {
    __typename
    account_to_channels_connection_count: count
    account_to_channels_connection_page_info: page_info {
        __typename
        page_info_has_next_page: has_next_page
        page_info_has_previous_page: has_previous_page
        page_info_start_cursor: start_cursor
        page_info_end_cursor: end_cursor
    }
    account_to_channels_connection_entities: entities {
        id
    }
}
";
