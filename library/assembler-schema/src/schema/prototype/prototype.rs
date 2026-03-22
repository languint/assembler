use crate::schema::prelude::*;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Prototype {
    #[serde(flatten)]
    pub basic_member: BasicMember,
    pub visibility: Option<Vec<Visibility>>,
    pub parent: Option<String>,
    pub r#abstract: bool,
    pub typename: Option<String>,
    pub instance_limit: Option<u64>,
    pub deprecated: bool,
    pub properties: Vec<Property>,
    pub custom_properties: Option<CustomProperties>,
}
