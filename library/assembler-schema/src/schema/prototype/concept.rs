use crate::schema::prelude::*;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PrototypeConcept {
    #[serde(flatten)]
    pub basic_member: BasicMember,
    pub parent: Option<String>,
    pub r#abstract: bool,
    pub inline: bool,
    #[serde(rename = "type")]
    pub ty: PrototypeType,
    pub properties: Option<Vec<Property>>,
}
