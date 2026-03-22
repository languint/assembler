use crate::schema::prelude::*;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DefineValue {
    pub name: String,
    pub order: u64,
    pub description: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Define {
    #[serde(flatten)]
    pub base: BasicMember,
    pub values: Option<Vec<DefineValue>>,
    pub subkeys: Option<Vec<Define>>,
}
