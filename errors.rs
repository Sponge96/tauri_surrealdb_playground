use serde::Serialize;
pub type MyResult<T> = std::result::Result<T, MyError>;

#[derive(Debug, Serialize)]
pub enum MyError {
    SurrealError(surrealdb::Error),
    StoreCreateFailed(String),
    StoreGetFailed(String),
    StoreUpdateFailed(String),
    StoreListFailed(String),
    StoreDeleteFailed(String),
    StoreFailToPatch {
        method: String,
        tb: String,
        tid: String,
    },
}

impl From<surrealdb::Error> for MyError {
    fn from(value: surrealdb::Error) -> Self {
        MyError::SurrealError(value)
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        match self {
            MyError::StoreFailToPatch { method, tb, tid } => {
                write!(fmt, "error in {method} for {tb}:{tid}")
            }
            _ => write!(fmt, "{self:?}"),
        }
    }
}

impl std::error::Error for MyError {}
