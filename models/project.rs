use super::types::{Castable, Creatable, Patchable};
use crate::errors::StoreError;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectGet {
    pub id: String,
    pub name: String,
    pub programs: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectGetMapping {
    pub id: Thing,
    pub name: String,
    pub programs: Vec<Thing>,
}

impl Castable for ProjectGetMapping {}

impl TryFrom<ProjectGetMapping> for ProjectGet {
    type Error = StoreError;

    fn try_from(response: ProjectGetMapping) -> Result<Self, Self::Error> {
        let id = response.id.to_raw();

        let programs = response
            .programs
            .into_iter()
            .map(|program| program.to_raw())
            .collect();

        Ok(ProjectGet {
            id,
            name: response.name,
            programs,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectCreate {
    pub name: String,
    pub programs: Vec<Thing>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectCreateMapping {
    pub name: String,
    pub programs: Vec<String>,
}

impl Castable for ProjectCreateMapping {}
impl Creatable for ProjectCreate {}

impl TryFrom<ProjectCreateMapping> for ProjectCreate {
    type Error = StoreError;

    fn try_from(response: ProjectCreateMapping) -> Result<Self, Self::Error> {
        let programs = response
            .programs
            .into_iter()
            .filter(|p| !p.is_empty())
            .map(|p| Thing::try_from(p).expect("shit"))
            .collect();

        Ok(ProjectCreate {
            name: response.name,
            programs,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub programs: Option<Vec<Thing>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectUpdateMapping {
    pub name: Option<String>,
    pub programs: Option<Vec<String>>,
}

impl Patchable for ProjectUpdate {}

impl TryFrom<ProjectUpdateMapping> for ProjectUpdate {
    type Error = StoreError;

    fn try_from(response: ProjectUpdateMapping) -> Result<Self, Self::Error> {
        let name = response.name;
        let programs = match response.programs {
            Some(programs) => {
                let programs: Vec<Thing> = programs
                    .into_iter()
                    .filter(|p| !p.is_empty())
                    .map(|p| Thing::try_from(p).expect("Failed to convert String to Thing"))
                    .collect();
                Some(programs)
            }
            None => None,
        };

        Ok(ProjectUpdate { name, programs })
    }
}
