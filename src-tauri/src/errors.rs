use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize, Error)]
pub enum StoreError {
    #[error("Database Error: {0}")]
    SurrealError(#[from] surrealdb::Error),
    #[error("Store Error: Failed to Create - '{0}'")]
    StoreCreateFailed(String),
    #[error("Store Error: Failed to List - '{0}'")]
    StoreListFailed(String),
    #[error("Store Error: Failed to Get - '{0}'")]
    StoreGetFailed(String),
    #[error("Store Error: Failed to Update - '{0}'")]
    StoreUpdateFailed(String),
    #[error("Store Error: Failed to Delete - '{0}'")]
    StoreDeleteFailed(String),
    #[error("Convert Error: Failed to Delete - '{0}'")]
    ConversionError(String),
}
