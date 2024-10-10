
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};
use crate::objects::api_token::ApiToken;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateApiTokenOutput {

    /// The API Token that has been created.
    #[serde (rename = "create_api_token_output_api_token")]
    pub api_token: ApiToken,

    /// The secret that should be used to authenticate against our API.
    // This secret is not stored and will never be available again after this. You must keep this secret secure as it grants access to your account.
    #[serde (rename = "create_api_token_output_client_secret")]
    pub client_secret: String,

}



pub const FRAGMENT: &str = "
fragment CreateApiTokenOutputFragment on CreateApiTokenOutput {
    __typename
    create_api_token_output_api_token: api_token {
        __typename
        api_token_id: id
        api_token_created_at: created_at
        api_token_updated_at: updated_at
        api_token_client_id: client_id
        api_token_name: name
        api_token_permissions: permissions
        api_token_is_deleted: is_deleted
    }
    create_api_token_output_client_secret: client_secret
}
";



