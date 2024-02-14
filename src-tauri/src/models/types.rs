use serde::{de::DeserializeOwned, Deserialize, Serialize};
use surrealdb::sql::Thing;

pub trait Castable: DeserializeOwned {}
pub trait Creatable: Serialize {}
pub trait Patchable: Serialize {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    pub id: Thing,
}

impl Record {
    pub fn get_id(&self) -> String {
        self.id.to_raw()
    }
}
