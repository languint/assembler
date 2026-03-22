use crate::schema::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Call(Method),
    Index(Attribute),
    Length(Attribute),
}

impl<'de> serde::Deserialize<'de> for Operator {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = serde_json::Value::deserialize(d)?;

        let name = raw["name"]
            .as_str()
            .ok_or_else(|| serde::de::Error::missing_field("name"))?;

        match name {
            "call" => Method::deserialize(raw)
                .map(Operator::Call)
                .map_err(serde::de::Error::custom),
            "index" => Attribute::deserialize(raw)
                .map(Operator::Index)
                .map_err(serde::de::Error::custom),
            "length" => Attribute::deserialize(raw)
                .map(Operator::Length)
                .map_err(serde::de::Error::custom),
            other => Err(serde::de::Error::unknown_variant(
                other,
                &["call", "index", "length"],
            )),
        }
    }
}
