use crate::schema::prelude::*;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Define {
    #[serde(flatten)]
    pub basic_member: BasicMember,
    pub values: Option<Vec<DefineValue>>,
    pub subkeys: Option<Vec<Define>>,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct DefineValue {
    pub name: String,
    pub order: u64,
    pub description: String,
}
