use crate::schema::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Type {
    Complex(Box<ComplexType>),
    Simple(String),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "complex_type", rename_all = "snake_case")]
pub enum ComplexType {
    Type {
        value: Type,
        description: String,
    },

    Union {
        options: Vec<Type>,
        #[serde(default)]
        full_format: bool,
    },

    Array {
        value: Type,
    },

    #[serde(alias = "LuaCustomTable")]
    Dictionary {
        key: Type,
        value: Type,
    },

    Table {
        parameters: Vec<Parameter>,
        #[serde(default)]
        variant_parameter_groups: Vec<ParameterGroup>,
        #[serde(default)]
        variant_parameter_description: Option<String>,
    },

    Tuple {
        values: Vec<Type>,
    },

    Function {
        parameters: Vec<Type>,
    },

    Literal {
        value: LiteralValue,
        #[serde(default)]
        description: Option<String>,
    },

    #[serde(rename = "LuaLazyLoadedValue")]
    LuaLazyLoadedValue {
        value: Type,
    },

    #[serde(rename = "LuaStruct")]
    LuaStruct {
        attributes: Vec<Attribute>,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum LiteralValue {
    String(String),
    Number(f64),
    Bool(bool),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn de(s: &str) -> Type {
        serde_json::from_str(s).unwrap()
    }

    #[test]
    fn simple() {
        assert!(matches!(de(r#""uint""#), Type::Simple(_)));
    }

    #[test]
    fn array() {
        let t = de(r#"{"complex_type":"array","value":"string"}"#);
        if let Type::Complex(inner) = t {
            assert!(matches!(*inner, ComplexType::Array { .. }));
        } else {
            panic!("expected Complex variant");
        }
    }

    #[test]
    fn lua_custom_table() {
        let t = de(r#"{"complex_type":"LuaCustomTable","key":"string","value":"uint"}"#);
        if let Type::Complex(inner) = t {
            assert!(matches!(*inner, ComplexType::Dictionary { .. }));
        } else {
            panic!("expected Complex variant");
        }
    }

    #[test]
    fn literal_bool() {
        let t = de(r#"{"complex_type":"literal","value":true}"#);
        if let Type::Complex(inner) = t {
            assert!(matches!(
                *inner,
                ComplexType::Literal {
                    value: LiteralValue::Bool(true),
                    ..
                }
            ));
        } else {
            panic!("expected Complex variant");
        }
    }
}
