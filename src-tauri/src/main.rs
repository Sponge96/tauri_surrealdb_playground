pub use errors::StoreError;
pub use models::Store;
mod api;
mod errors;
mod models;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), StoreError> {
    let store = Store::new(
        "127.0.0.1:8000".to_string(),
        "test".to_string(),
        "test".to_string(),
    )
    .await?;
    let store = Arc::new(store);

    tauri::Builder::default()
        .manage(store)
        .invoke_handler(tauri::generate_handler![
            api::get_project,
            api::delete_project,
            api::update_project,
            api::create_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
