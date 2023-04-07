// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved

use serde::Deserialize;

/// This is a wrapper struct for nested entities. The graphql query for object only query for the
/// entity ID if the entity is a field within another object. This struct derives from Deserialize
/// for easier json deserialization.
#[derive(Clone, Deserialize)]
pub struct EntityWrapper {
    pub id: String,
}
