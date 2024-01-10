
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RichText {

    
    #[serde (rename = "rich_text_text")]
    pub text: String,

}



pub const FRAGMENT: &str = "
fragment RichTextFragment on RichText {
    __typename
    rich_text_text: text
}
";



