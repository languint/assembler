use crate::schema::prelude::*;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RuntimeConcept {
    #[serde(flatten)]
    pub basic_member: BasicMember,
    #[serde(rename = "type")]
    pub ty: RuntimeType,
}
