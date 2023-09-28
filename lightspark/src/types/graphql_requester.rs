use async_trait::async_trait;
use serde_json::Value;

use crate::error::Error;

#[async_trait]
pub trait GraphQLRequester: Send + Sync {
    async fn execute_graphql(
        &self,
        operation: &str,
        variables: Option<Value>,
    ) -> Result<Value, Error>;
}
