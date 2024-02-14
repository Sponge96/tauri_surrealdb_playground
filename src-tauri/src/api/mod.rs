use crate::errors::StoreError;
use crate::models::project::{Project, ProjectController, ProjectCreate, ProjectUpdate};
use crate::models::Store;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Manager, Wry};

#[derive(Serialize)]
pub struct IpcResponse<D>
where
    D: Serialize,
{
    error: Option<IpcError>,
    result: Option<IpcSimpleResult<D>>,
}

#[derive(Serialize)]
struct IpcError {
    message: String,
}

#[derive(Serialize)]
pub struct IpcSimpleResult<D>
where
    D: Serialize,
{
    pub data: D,
}

impl<D> From<Result<D, StoreError>> for IpcResponse<D>
where
    D: Serialize,
{
    fn from(res: Result<D, StoreError>) -> Self {
        match res {
            Ok(data) => IpcResponse {
                error: None,
                result: Some(IpcSimpleResult { data }),
            },
            Err(err) => IpcResponse {
                error: Some(IpcError {
                    message: format!("{err}"),
                }),
                result: None,
            },
        }
    }
}

#[derive(Deserialize)]
pub struct GetParams {
    pub id: String,
}

#[derive(Deserialize)]
pub struct UpdateParams<D> {
    pub id: String,
    pub data: D,
}

#[derive(Deserialize)]
pub struct CreateParams<D> {
    pub id: String,
    pub data: D,
}

/// TODO: implement this without MyResult
// #[tauri::command]
// pub async fn list_projects(app: AppHandle<Wry>) -> IpcResponse<Vec<Project>> {
//     let store = (*app.state::<Arc<Store>>()).clone();
//     let result = ProjectController::list(store).await;

//     IpcResponse::from(result.map(|projects| projects.into_iter().flatten().collect()))
// }

#[tauri::command]
pub async fn get_project(app: AppHandle<Wry>, args: GetParams) -> IpcResponse<Project> {
    let store = (*app.state::<Arc<Store>>()).clone();
    ProjectController::get(store, &args.id).await.into()
}

#[tauri::command]
pub async fn delete_project(app: AppHandle<Wry>, args: GetParams) -> IpcResponse<String> {
    let store = (*app.state::<Arc<Store>>()).clone();
    ProjectController::delete(store, &args.id).await.into()
}

#[tauri::command]
pub async fn update_project(
    app: AppHandle<Wry>,
    args: UpdateParams<ProjectUpdate>,
) -> IpcResponse<String> {
    let store = (*app.state::<Arc<Store>>()).clone();
    ProjectController::update(store, &args.id, args.data)
        .await
        .into()
}

#[tauri::command]
pub async fn create_project(
    app: AppHandle<Wry>,
    args: CreateParams<ProjectCreate>,
) -> IpcResponse<String> {
    let store = (*app.state::<Arc<Store>>()).clone();
    ProjectController::create(store, &args.id, args.data)
        .await
        .into()
}
