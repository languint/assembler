use crate::schema::prelude::*;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(untagged)]
pub enum RuntimeType {
    Complex(Box<RuntimeComplexType>),
    Simple(String),
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(tag = "complex_type", rename_all = "snake_case")]
pub enum RuntimeComplexType {
    Type {
        value: RuntimeType,
        description: String,
    },
    Union {
        options: Vec<RuntimeType>,
        full_format: bool,
    },
    Array {
        value: RuntimeType,
    },
    #[serde(alias = "LuaCustomTable")]
    Dictionary {
        key: RuntimeType,
        value: RuntimeType,
    },
    Table {
        parameters: Vec<Parameter>,
        #[serde(default)]
        variant_parameter_groups: Option<Vec<ParameterGroup>>,
        #[serde(default)]
        variant_parameter_description: Option<String>,
    },
    Tuple {
        values: Vec<RuntimeType>,
    },
    Function {
        parameters: Vec<RuntimeType>,
    },
    Literal {
        value: RuntimeLiteralValue,
        #[serde(default)]
        description: Option<String>,
    },
    #[serde(rename = "LuaLazyLoadedValue")]
    LuaLazyLoadedValue {
        value: RuntimeType,
    },
    #[serde(rename = "LuaStruct")]
    LuaStruct {
        attributes: Vec<Attribute>,
    },
    Builtin,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(untagged)]
pub enum RuntimeLiteralValue {
    String(String),
    Number(f64),
    Bool(bool),
}
