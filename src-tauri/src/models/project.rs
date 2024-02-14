use super::types::{Castable, Creatable, Patchable};
use crate::errors::StoreError;
use crate::Store;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with_macros::skip_serializing_none;
use std::sync::Arc;
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    #[serde(deserialize_with = "de_thing_to_string")]
    pub id: String,
    pub name: String,
    #[serde(deserialize_with = "de_programs_to_string")]
    pub programs: Vec<String>,
}

impl Castable for Project {}

fn de_programs_to_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize the Vec<Thing> into a Vec<String>
    let things: Vec<Thing> = Deserialize::deserialize(deserializer)?;

    // Convert each Thing to a String using the to_raw() method
    let strings: Vec<String> = things.into_iter().map(|thing| thing.to_raw()).collect();

    Ok(strings)
}

fn de_thing_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let thing: Thing = Deserialize::deserialize(deserializer)?;

    Ok(thing.to_raw())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectCreate {
    pub name: String,
    pub programs: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectCreateTest {
    pub name: String,
    pub programs: Vec<Thing>,
}

impl From<ProjectCreate> for ProjectCreateTest {
    fn from(project_create: ProjectCreate) -> Self {
        ProjectCreateTest {
            name: project_create.name,
            programs: project_create
                .programs
                .into_iter()
                .filter(|p| !p.is_empty())
                .map(|p| Thing::try_from(p).expect("Failed to convert Program to Thing"))
                .collect(),
        }
    }
}
impl Creatable for ProjectCreate {}
impl Creatable for ProjectCreateTest {}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectUpdate {
    pub name: Option<String>,
    pub programs: Option<Vec<String>>,
}

impl Patchable for ProjectUpdate {}

pub struct ProjectController;

impl ProjectController {
    const RECORD: &'static str = "project";

    // pub async fn get(store: Arc<Store>, id: &str) -> Result<Project, StoreError> {
    //     store.get().get::<Project>(Self::RECORD, id).await?
    // }
    pub async fn get(store: Arc<Store>, id: &str) -> Result<Project, StoreError> {
        store
            .get()
            .get::<Project>(Self::RECORD, id)
            .await
            .map_err(|err| StoreError::from(err))
    }

    pub async fn create(
        store: Arc<Store>,
        id: &str,
        data: ProjectCreate,
    ) -> Result<String, StoreError> {
        let data_test: ProjectCreateTest = data.into(); // Convert ProjectCreate to ProjectCreateTest
        Ok(store
            .get()
            .create(Self::RECORD, id, data_test)
            .await?
            .get_id())
    }

    pub async fn update(
        store: Arc<Store>,
        id: &str,
        data: ProjectUpdate,
    ) -> Result<String, StoreError> {
        Ok(store.get().update(Self::RECORD, id, data).await?.get_id())
    }

    pub async fn delete(store: Arc<Store>, id: &str) -> Result<String, StoreError> {
        Ok(store.get().delete(Self::RECORD, id).await?.get_id())
    }

    // pub async fn list(store: Arc<Store>) -> Result<Vec<Option<Project>>, StoreError> {
    //     let project_responses = store.get().list::<ProjectResponse>(Self::RECORD).await?;

    //     Ok(project_responses
    //         .into_iter()
    //         .map(|response| response.map(Project::try_from).transpose())
    //         .collect::<Result<Vec<Option<Project>>, _>>()?)
    // }
}
