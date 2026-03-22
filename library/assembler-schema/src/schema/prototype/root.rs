use crate::schema::prelude::*;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PrototypeApiRoot {
    pub application: String,
    pub application_version: String,
    pub api_version: u64,
    pub stage: Stage,
    pub prototypes: Vec<Prototype>,
    pub types: Vec<PrototypeConcept>,
    pub defines: Vec<Define>,
}
