use serde::Serialize;
pub type MyResult<T> = std::result::Result<T, MyError>;

#[derive(Debug, Serialize)]
pub enum MyError {
    SurrealError(surrealdb::Error),
    StoreCreateFailed(String),
    StoreGetFailed(String),
    StoreUpdateFailed(String),
    StoreListFailed(String),
}

impl From<surrealdb::Error> for MyError {
    fn from(value: surrealdb::Error) -> Self {
        MyError::SurrealError(value)
    }
}
