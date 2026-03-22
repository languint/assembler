use crate::schema::prelude::*;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Parameter {
    pub name: String,
    pub order: u64,
    pub description: String,
    pub r#type: Type,
    pub optional: bool,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ParameterGroup {
    pub name: String,
    pub order: u64,
    pub r#type: Type,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct VariadicParameter {
    pub r#type: Option<Type>,
    pub description: Option<String>,
}
