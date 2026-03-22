use crate::schema::prelude::*;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Concept {
    #[serde(flatten)]
    pub base: BasicMember,
    pub r#type: Type,
}
