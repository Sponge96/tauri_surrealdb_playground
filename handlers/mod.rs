use crate::controllers::project::ProjectController;
use crate::errors::MyResult;
use crate::models::project::Project;
use crate::models::Store;
use serde::Deserialize;
use std::sync::Arc;
use tauri::{AppHandle, Manager, Wry};

#[derive(Deserialize)]
pub struct GetParams {
    pub record: String,
    pub id: String,
}

#[tauri::command]
pub async fn greet(app: AppHandle<Wry>, args: GetParams) -> MyResult<Project> {
    println!("test");
    let store = (*app.state::<Arc<Store>>()).clone();
    let result = ProjectController::get(store, &args.id).await;
    result.into()
}

#[tauri::command]
pub async fn list_projects(app: AppHandle<Wry>) -> MyResult<Vec<Project>> {
    println!("listing projects");
    let store = (*app.state::<Arc<Store>>()).clone();
    let result = ProjectController::list(store).await;
    result.into()
}
