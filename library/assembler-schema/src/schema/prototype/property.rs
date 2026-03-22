use crate::schema::prelude::*;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum PropertyDefault {
    Text(String),
    Literal(PrototypeType),
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Property {
    #[serde(flatten)]
    pub basic_member: BasicMember,
    pub visibility: Option<Vec<Visibility>>,
    pub alt_name: Option<String>,
    pub r#override: bool,
    #[serde(rename = "type")]
    pub ty: PrototypeType,
    pub optional: bool,
    pub default: Option<PropertyDefault>,
}
