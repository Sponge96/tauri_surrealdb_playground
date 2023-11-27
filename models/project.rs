use super::types::{Castable, Creatable, Patchable, RecordId};
use crate::errors::{MyError, MyResult};
use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub programs: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectResponse {
    pub id: RecordId,
    pub name: String,
    pub programs: Option<Vec<RecordId>>,
}

impl Castable for ProjectResponse {}

impl TryFrom<ProjectResponse> for Project {
    type Error = MyError;

    fn try_from(value: ProjectResponse) -> MyResult<Project> {
        let programs = value
            .programs
            .map(|ids| ids.into_iter().map(|id| id.get_id()).collect());

        let task = Project {
            id: value.id.get_id(),
            name: value.name,
            programs,
        };

        Ok(task)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectCreate {
    pub name: String,
    pub programs: Option<Vec<String>>,
}

impl Creatable for ProjectCreate {}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectUpdate {
    pub name: Option<String>,
    pub programs: Option<Vec<String>>,
}

impl Patchable for ProjectUpdate {}
