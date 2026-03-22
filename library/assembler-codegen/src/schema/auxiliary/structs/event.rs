use crate::schema::prelude::*;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Event {
    #[serde(flatten)]
    pub base: BasicMember,
    pub data: Vec<Parameter>,
    pub filter: Option<String>,
}
