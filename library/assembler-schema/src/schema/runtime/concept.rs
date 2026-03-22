use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct RuntimeConcept {
    #[serde(flatten)]
    pub basic_member: BasicMember,
    #[serde(rename = "type")]
    pub ty: RuntimeType,
}
