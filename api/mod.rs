use crate::errors::StoreError;
use crate::models::project::{ProjectCreateMapping, ProjectGet, ProjectUpdateMapping};
use crate::models::project_handler::ProjectHandler;
use crate::models::Client;
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

#[tauri::command]
pub async fn get_project(app: AppHandle<Wry>, args: GetParams) -> IpcResponse<ProjectGet> {
    let client = (*app.state::<Arc<Client>>()).clone();
    ProjectHandler::get(client, &args.id).await.into()
}

#[tauri::command]
pub async fn create_project(
    app: AppHandle<Wry>,
    args: CreateParams<ProjectCreateMapping>,
) -> IpcResponse<String> {
    let client = (*app.state::<Arc<Client>>()).clone();
    ProjectHandler::create(client, &args.id, args.data)
        .await
        .into()
}

#[tauri::command]
pub async fn update_project(
    app: AppHandle<Wry>,
    args: UpdateParams<ProjectUpdateMapping>,
) -> IpcResponse<String> {
    let client = (*app.state::<Arc<Client>>()).clone();
    ProjectHandler::update(client, &args.id, args.data)
        .await
        .into()
}

#[tauri::command]
pub async fn delete_project(app: AppHandle<Wry>, args: GetParams) -> IpcResponse<String> {
    let client = (*app.state::<Arc<Client>>()).clone();
    ProjectHandler::delete(client, &args.id).await.into()
}

#[tauri::command]
pub async fn list_projects(app: AppHandle<Wry>) -> IpcResponse<Vec<ProjectGet>> {
    let client = (*app.state::<Arc<Client>>()).clone();
    ProjectHandler::list(client).await.into()
}
