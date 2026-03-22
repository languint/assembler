use crate::schema::prelude::*;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Parameter {
    pub name: String,
    pub order: u64,
    pub description: String,
    #[serde(rename = "type")]
    pub ty: RuntimeType,
    pub optional: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct ParameterGroup {
    pub name: String,
    pub order: u64,
    pub description: String,
    pub parameters: Vec<Parameter>,
}
