use crate::schema::prelude::*;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TopLevelMembers {
    pub application: String,
    pub application_version: String,
    pub api_version: u64,
    pub stage: Stage,
}
