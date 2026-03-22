use crate::schema::prelude::*;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "name", rename_all = "lowercase")]
pub enum Operator {
    Call(Method),
    Index(Attribute),
    Length(Attribute),
}

