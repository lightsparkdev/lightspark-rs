// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use serde::de::DeserializeOwned;

pub trait GetEntity: DeserializeOwned {
    fn get_entity_query() -> String;
}
