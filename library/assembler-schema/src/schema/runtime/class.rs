use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Class {
    #[serde(flatten)]
    pub basic_member: BasicMember,
    pub visibility: Option<Vec<Visibility>>,
    pub parent: Option<String>,
    pub r#abstract: bool,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
    pub operators: Vec<Operator>,
}
