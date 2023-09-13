// Copyright ©, 2023-present, Lightspark Group, Inc. - All Rights Reserved
use crate::objects::entity::Entity;
use crate::types::custom_date_formats::custom_date_format;
use crate::types::get_entity::GetEntity;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Signable {
    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    #[serde(rename = "signable_id")]
    pub id: String,

    /// The date and time when the entity was first created.
    #[serde(with = "custom_date_format", rename = "signable_created_at")]
    pub created_at: DateTime<Utc>,

    /// The date and time when the entity was last updated.
    #[serde(with = "custom_date_format", rename = "signable_updated_at")]
    pub updated_at: DateTime<Utc>,
}

impl Entity for Signable {
    /// The unique identifier of this entity across all Lightspark systems. Should be treated as an opaque string.
    fn get_id(&self) -> String {
        self.id.clone()
    }

    /// The date and time when the entity was first created.
    fn get_created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// The date and time when the entity was last updated.
    fn get_updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    fn type_name(&self) -> &'static str {
        "Signable"
    }
}

impl GetEntity for Signable {
    fn get_entity_query() -> String {
        format!(
            "
        query GetEntity($id: ID!) {{
            entity(id: $id) {{
                ... on Signable {{
                    ... SignableFragment
                }}
            }}
        }}

        {}",
            FRAGMENT
        )
    }
}

pub const FRAGMENT: &str = "
fragment SignableFragment on Signable {
    __typename
    signable_id: id
    signable_created_at: created_at
    signable_updated_at: updated_at
}
";
