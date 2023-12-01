use super::project::{
    ProjectCreate, ProjectCreateMapping, ProjectGet, ProjectGetMapping, ProjectUpdate,
    ProjectUpdateMapping,
};
use crate::errors::StoreError;
use crate::Client;
use std::sync::Arc;

pub struct ProjectHandler;

impl ProjectHandler {
    const RECORD: &'static str = "project";

    pub async fn get(client: Arc<Client>, id: &str) -> Result<ProjectGet, StoreError> {
        client
            .get()
            .get::<ProjectGetMapping>(Self::RECORD, id)
            .await?
            .try_into()
    }

    pub async fn create(
        client: Arc<Client>,
        id: &str,
        data: ProjectCreateMapping,
    ) -> Result<String, StoreError> {
        let data_mapping = ProjectCreate::try_from(data)?;

        Ok(client
            .get()
            .create(Self::RECORD, id, data_mapping)
            .await?
            .to_raw())
    }

    pub async fn update(
        client: Arc<Client>,
        id: &str,
        data: ProjectUpdateMapping,
    ) -> Result<String, StoreError> {
        let data_update = ProjectUpdate::try_from(data)?;

        if data_update.name.is_none() && data_update.programs.is_none() {
            return Ok(String::from("No update performed"));
        }

        Ok(client
            .get()
            .update(Self::RECORD, id, data_update)
            .await?
            .to_raw())
    }

    pub async fn delete(client: Arc<Client>, id: &str) -> Result<String, StoreError> {
        Ok(client.get().delete(Self::RECORD, id).await?.to_raw())
    }

    pub async fn list(client: Arc<Client>) -> Result<Vec<ProjectGet>, StoreError> {
        let project_mappings = client.get().list::<ProjectGetMapping>(Self::RECORD).await?;

        let projects: Vec<ProjectGet> = project_mappings
            .into_iter()
            .map(|project_mapping| project_mapping.try_into())
            .collect::<Result<Vec<ProjectGet>, StoreError>>()?;

        Ok(projects)
    }
}
