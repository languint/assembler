use crate::prelude::*;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CustomProperties {
    pub description: String,
    pub lists: Option<Vec<String>>,
    pub examples: Option<Vec<String>>,
    pub images: Option<Vec<Image>>,
    pub key_type: PrototypeType,
    pub value_type: PrototypeType,
}
