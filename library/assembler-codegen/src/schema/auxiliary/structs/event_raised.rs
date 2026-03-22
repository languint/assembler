use crate::schema::prelude::*;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct EventRaised {
    pub name: String,
    pub order: u64,
    pub description: String,
    pub timeframe: TimeFrame,
    pub optional: bool,
}
