
// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::entity::Entity;
use serde_json::Value;
use serde::{Deserialize, Deserializer, Serialize};
use super::api_token::ApiToken;

pub trait AuditLogActor : Entity {


fn type_name(&self) -> &'static str;
}


#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize)]
pub enum AuditLogActorEnum {
    ApiToken(ApiToken),

}



impl<'de> Deserialize<'de> for AuditLogActorEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        if let Some(typename) = value.get("__typename").and_then(Value::as_str) {
            match typename {
                "ApiToken" => {
                    let obj = ApiToken::deserialize(value)
                        .map_err(|err|
                            serde::de::Error::custom(format!("Serde JSON Error {}", err))
                        )?;
                    Ok(AuditLogActorEnum::ApiToken(obj))
                },

                _ => Err(serde::de::Error::custom("unknown typename")),
            }
        } else {
            Err(serde::de::Error::custom("missing __typename field on AuditLogActor"))
        }
    }
}

