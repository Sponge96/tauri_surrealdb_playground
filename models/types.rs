use serde::{de::DeserializeOwned, Serialize};

pub trait Castable: DeserializeOwned {}
pub trait Creatable: Serialize {}
pub trait Patchable: Serialize {}
