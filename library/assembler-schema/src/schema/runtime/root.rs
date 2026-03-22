use crate::schema::prelude::*;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RuntimeApiRoot {
    pub application: String,
    pub application_version: String,
    pub api_version: u64,
    pub stage: Stage,
    pub classes: Vec<Class>,
    pub events: Vec<Event>,
    pub concepts: Vec<RuntimeConcept>,
    pub defines: Vec<Define>,
    pub global_objects: Vec<Parameter>,
    pub global_functions: Vec<Method>,
}
