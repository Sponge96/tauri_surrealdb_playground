use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait Castable: DeserializeOwned {}
pub trait Creatable: Serialize {}
pub trait Patchable: Serialize {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    pub id: RecordId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordId(surrealdb::sql::Thing);

impl RecordId {
    pub fn get_id(&self) -> String {
        self.0.to_raw()
    }
}
