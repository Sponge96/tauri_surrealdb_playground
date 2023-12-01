pub use errors::StoreError;
pub use models::Client;
mod api;
mod errors;
mod models;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), StoreError> {
    let client = Client::new(
        "127.0.0.1:8000".to_string(),
        "test".to_string(),
        "test".to_string(),
    )
    .await?;
    let client = Arc::new(client);

    tauri::Builder::default()
        .manage(client)
        .invoke_handler(tauri::generate_handler![
            api::get_project,
            api::create_project,
            api::delete_project,
            api::update_project,
            api::list_projects,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
