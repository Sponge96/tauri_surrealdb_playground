pub use errors::{MyError, MyResult};
pub use models::Store;
mod controllers;
mod errors;
mod models;
use std::sync::Arc;
mod handlers;

#[tokio::main]
async fn main() -> MyResult<()> {
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
            handlers::get_project,
            handlers::list_projects,
            handlers::delete_project,
            handlers::update_project,
            handlers::create_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
