pub mod project;
mod store;
mod types;
use crate::errors::MyResult;
use store::SurrealDbStore;

pub struct Store(SurrealDbStore);

impl Store {
    pub async fn new(address: String, ns: String, db: String) -> MyResult<Self> {
        Ok(Store(SurrealDbStore::new(address, ns, db).await?))
    }

    pub fn get(&self) -> &SurrealDbStore {
        &self.0
    }
}
