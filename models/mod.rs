pub mod project;
pub mod project_handler;
mod surreal_client;
mod types;
use crate::errors::StoreError;
use surreal_client::SurrealDbClient;

pub struct Client(SurrealDbClient);

impl Client {
    pub async fn new(address: String, ns: String, db: String) -> Result<Self, StoreError> {
        Ok(Client(SurrealDbClient::new(address, ns, db).await?))
    }

    pub fn get(&self) -> &SurrealDbClient {
        &self.0
    }
}
