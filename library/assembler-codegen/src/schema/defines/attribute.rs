use crate::schema::prelude::*;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Attribute {
    pub visibility: Option<Visibility>,
    pub raises: Option<Vec<EventRaised>>,
    pub subclasses: Option<Vec<String>>,
    pub read_type: Option<Type>,
    pub write_type: Option<Type>,
    pub optional: bool,
}
