use super::types::{Castable, Creatable, Patchable};
use crate::errors::StoreError;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

pub struct SurrealDbClient {
    pub database: Surreal<Client>,
}

impl SurrealDbClient {
    pub(in crate::models) async fn new(
        address: String,
        ns: String,
        db: String,
    ) -> Result<Self, StoreError> {
        let database = Surreal::new::<Ws>(address).await?;
        database.use_ns(ns).use_db(db).await?;
        Ok(SurrealDbClient { database })
    }

    pub(in crate::models) async fn get<T: Castable>(
        &self,
        record: &str,
        id: &str,
    ) -> Result<T, StoreError> {
        self.database
            .select((record, id))
            .await?
            .ok_or_else(|| StoreError::StoreGetFailed(format!("record: {}, id: {}", record, id)))
    }

    pub(in crate::models) async fn create<T: Creatable>(
        &self,
        record: &str,
        id: &str,
        data: T,
    ) -> Result<Thing, StoreError> {
        self.database
            .create((record, id))
            .content(data)
            .await?
            .ok_or(StoreError::StoreCreateFailed(format!(
                "record: {}, id: {}",
                record, id
            )))
    }

    pub(in crate::models) async fn update<T: Patchable>(
        &self,
        record: &str,
        id: &str,
        data: T,
    ) -> Result<Thing, StoreError> {
        self.database
            .update((record, id))
            .merge(data)
            .await?
            .ok_or(StoreError::StoreUpdateFailed(format!(
                "record: {}, id: {}",
                record, id
            )))
    }

    pub(in crate::models) async fn delete(
        &self,
        record: &str,
        id: &str,
    ) -> Result<Thing, StoreError> {
        self.database
            .delete((record, id))
            .await?
            .ok_or(StoreError::StoreDeleteFailed(format!(
                "record: {}, id: {}",
                record, id
            )))
    }

    pub(in crate::models) async fn list<T: Castable>(
        &self,
        record: &str,
    ) -> Result<Vec<T>, StoreError> {
        let records: Vec<T> = self.database.select(record).await?;
        Ok(records)
    }
}
