use crate::errors::MyResult;
use crate::models::project::{Project, ProjectCreate, ProjectResponse, ProjectUpdate};
use crate::Store;
use std::sync::Arc;

pub struct ProjectController;

impl ProjectController {
    const RECORD: &'static str = "project";

    pub async fn get(store: Arc<Store>, id: &str) -> MyResult<Project> {
        store
            .get()
            .execute_get::<ProjectResponse>(Self::RECORD, id)
            .await?
            .try_into()
    }

    pub async fn create(store: Arc<Store>, id: &str, data: ProjectCreate) -> MyResult<String> {
        Ok(store
            .get()
            .execute_create(Self::RECORD, id, data)
            .await?
            .id
            .get_id())
    }

    pub async fn update(store: Arc<Store>, id: &str, data: ProjectUpdate) -> MyResult<String> {
        Ok(store
            .get()
            .execute_update(Self::RECORD, id, data)
            .await?
            .id
            .get_id())
    }

    pub async fn delete(store: Arc<Store>, id: &str) -> MyResult<String> {
        Ok(store
            .get()
            .execute_delete(Self::RECORD, id)
            .await?
            .id
            .get_id())
    }

    pub async fn list(store: Arc<Store>) -> MyResult<Vec<Project>> {
        let project_responses = store
            .get()
            .execute_list::<ProjectResponse>(Self::RECORD)
            .await?;

        project_responses
            .into_iter()
            .map(Project::try_from)
            .collect()
    }
}
