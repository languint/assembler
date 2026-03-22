use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Event {
    #[serde(flatten)]
    pub basic_member: BasicMember,
    pub data: Vec<Parameter>,
    pub filter: Option<String>,
}
