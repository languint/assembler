use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Parameter {
    /// `serde::default` needed because [`Method::return_values`] do not specify a `name`
    #[serde(default)]
    pub name: String,
    pub order: u64,
    pub description: String,
    #[serde(rename = "type")]
    pub ty: RuntimeType,
    /// Absent on [`RuntimeApiRoot::global_objects`]
    #[serde(default)]
    pub optional: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct ParameterGroup {
    pub name: String,
    pub order: u64,
    pub description: String,
    pub parameters: Vec<Parameter>,
}
