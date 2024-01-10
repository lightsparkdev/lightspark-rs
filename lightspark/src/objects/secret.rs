
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Secret {

    
    #[serde (rename = "secret_encrypted_value")]
    pub encrypted_value: String,

    
    #[serde (rename = "secret_cipher")]
    pub cipher: String,

}



pub const FRAGMENT: &str = "
fragment SecretFragment on Secret {
    __typename
    secret_encrypted_value: encrypted_value
    secret_cipher: cipher
}
";



