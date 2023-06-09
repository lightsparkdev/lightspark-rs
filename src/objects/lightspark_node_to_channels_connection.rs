// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::channel::Channel;
use crate::objects::page_info::PageInfo;
use serde::Deserialize;
use std::vec::Vec;

#[derive(Deserialize)]
pub struct LightsparkNodeToChannelsConnection {
    /// An object that holds pagination information about the objects in this connection.
    #[serde(rename = "lightspark_node_to_channels_connection_page_info")]
    pub page_info: PageInfo,

    /// The total count of objects in this connection, using the current filters. It is different from the number of objects returned in the current page (in the `entities` field).
    #[serde(rename = "lightspark_node_to_channels_connection_count")]
    pub count: i64,

    /// The channels for the current page of this connection.
    #[serde(rename = "lightspark_node_to_channels_connection_entities")]
    pub entities: Vec<Channel>,
}

pub const FRAGMENT: &str = "
fragment LightsparkNodeToChannelsConnectionFragment on LightsparkNodeToChannelsConnection {
    __typename
    lightspark_node_to_channels_connection_page_info: page_info {
        __typename
        page_info_has_next_page: has_next_page
        page_info_has_previous_page: has_previous_page
        page_info_start_cursor: start_cursor
        page_info_end_cursor: end_cursor
    }
    lightspark_node_to_channels_connection_count: count
    lightspark_node_to_channels_connection_entities: entities {
        id
    }
}
";
