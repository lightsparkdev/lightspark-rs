// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::channel::Channel;
use crate::objects::connection::Connection;
use crate::objects::page_info::PageInfo;
use serde::Deserialize;
use std::vec::Vec;

#[derive(Clone, Deserialize)]
pub struct LightsparkNodeToChannelsConnection {
    /// The total count of objects in this connection, using the current filters. It is different from the number of objects returned in the current page (in the `entities` field).
    #[serde(rename = "lightspark_node_to_channels_connection_count")]
    pub count: i64,

    /// An object that holds pagination information about the objects in this connection.
    #[serde(rename = "lightspark_node_to_channels_connection_page_info")]
    pub page_info: PageInfo,

    /// The channels for the current page of this connection.
    #[serde(rename = "lightspark_node_to_channels_connection_entities")]
    pub entities: Vec<Channel>,
}

impl Connection for LightsparkNodeToChannelsConnection {
    /// The total count of objects in this connection, using the current filters. It is different from the number of objects returned in the current page (in the `entities` field).
    fn get_count(&self) -> i64 {
        self.count
    }

    /// An object that holds pagination information about the objects in this connection.
    fn get_page_info(&self) -> PageInfo {
        self.page_info.clone()
    }

    fn type_name(&self) -> &'static str {
        "LightsparkNodeToChannelsConnection"
    }
}

pub const FRAGMENT: &str = "
fragment LightsparkNodeToChannelsConnectionFragment on LightsparkNodeToChannelsConnection {
    __typename
    lightspark_node_to_channels_connection_count: count
    lightspark_node_to_channels_connection_page_info: page_info {
        __typename
        page_info_has_next_page: has_next_page
        page_info_has_previous_page: has_previous_page
        page_info_start_cursor: start_cursor
        page_info_end_cursor: end_cursor
    }
    lightspark_node_to_channels_connection_entities: entities {
        id
    }
}
";
