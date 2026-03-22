use crate::schema::prelude::*;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Method {
    #[serde(flatten)]
    pub basic_member: BasicMember,
    pub visibility: Option<Vec<Visibility>>,
    pub raises: Option<Vec<EventRaised>>,
    pub subclasses: Option<Vec<String>>,
    pub parameters: Vec<Parameter>,
    pub variant_parameter_groups: Option<Vec<ParameterGroup>>,
    pub variant_parameter_description: Option<String>,
    pub variadic_parameter: Option<VariadicParameter>,
    pub format: MethodFormat,
    pub return_values: Vec<Parameter>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct VariadicParameter {
    #[serde(rename = "type")]
    pub ty: Option<RuntimeType>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct MethodFormat {
    pub takes_table: bool,
    pub table_optional: Option<bool>,
}
