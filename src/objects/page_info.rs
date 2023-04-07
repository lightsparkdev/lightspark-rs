// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PageInfo {
    #[serde(rename = "page_info_has_next_page")]
    pub has_next_page: Option<bool>,

    #[serde(rename = "page_info_has_previous_page")]
    pub has_previous_page: Option<bool>,

    #[serde(rename = "page_info_start_cursor")]
    pub start_cursor: Option<String>,

    #[serde(rename = "page_info_end_cursor")]
    pub end_cursor: Option<String>,
}

pub const FRAGMENT: &str = "
fragment PageInfoFragment on PageInfo {
    __typename
    page_info_has_next_page: has_next_page
    page_info_has_previous_page: has_previous_page
    page_info_start_cursor: start_cursor
    page_info_end_cursor: end_cursor
}
";
