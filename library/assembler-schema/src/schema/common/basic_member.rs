use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct BasicMember {
    pub name: String,
    pub order: u64,
    pub description: String,
    pub lists: Option<Vec<String>>,
    pub examples: Option<Vec<String>>,
    pub images: Option<Vec<Image>>,
}
