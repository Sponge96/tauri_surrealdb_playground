use super::types::{Castable, Creatable, Patchable, Record};
use crate::errors::{MyError, MyResult};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;

pub struct SurrealDbStore {
    pub database: Surreal<Client>,
}

impl SurrealDbStore {
    pub async fn new(address: String, ns: String, db: String) -> MyResult<Self> {
        let database = Surreal::new::<Ws>(address).await?;
        database.use_ns(ns).use_db(db).await?;
        Ok(SurrealDbStore { database })
    }

    pub async fn execute_get<T: Castable>(&self, record: &str, id: &str) -> MyResult<T> {
        let result: Option<T> = self.database.select((record, id)).await?;
        result.ok_or(MyError::StoreGetFailed(format!(
            "execute_get {record}:{id} failed."
        )))
    }

    pub async fn execute_list<T: Castable>(&self, record: &str) -> MyResult<Vec<T>> {
        let result: Vec<T> = self.database.select(record).await?;
        // TODO: can this propergate an error? I'm assuming only a SurrealDb one
        Ok(result)
    }

    pub async fn execute_create<T: Creatable>(
        &self,
        record: &str,
        id: &str,
        data: T,
    ) -> MyResult<Record> {
        let result: Option<Record> = self.database.create((record, id)).content(data).await?;
        result
            .into_iter()
            .next()
            .ok_or(MyError::StoreCreateFailed(format!(
                "execute_create {record}:{id} failed."
            )))
    }

    pub async fn execute_update<T: Patchable>(
        &self,
        record: &str,
        id: &str,
        data: T,
    ) -> MyResult<Record> {
        let result: Option<Record> = self.database.update((record, id)).merge(data).await?;
        result.ok_or(MyError::StoreUpdateFailed(format!(
            "execute_update {record}:{id} failed."
        )))
    }

    pub async fn execute_delete(&self, record: &str, id: &str) -> MyResult<Record> {
        let result: Option<Record> = self.database.delete((record, id)).await?;
        result.ok_or(MyError::StoreDeleteFailed(format!("del failed")))
    }
}
