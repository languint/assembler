use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Attribute {
    #[serde(flatten)]
    pub basic_member: BasicMember,
    pub visibility: Option<Vec<Visibility>>,
    pub raises: Option<Vec<EventRaised>>,
    pub subclasses: Option<Vec<String>>,
    pub read_type: Option<RuntimeType>,
    pub write_type: Option<RuntimeType>,
    pub optional: bool,
}
