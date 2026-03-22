#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum PrototypeType {
    Complex(Box<PrototypeComplexType>),
    Simple(String),
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "complex_type", rename_all = "snake_case")]
pub enum PrototypeComplexType {
    Array {
        value: PrototypeType,
    },
    Dictionary {
        key: PrototypeType,
        value: PrototypeType,
    },
    Tuple {
        values: Vec<PrototypeType>,
    },
    Union {
        options: Vec<PrototypeType>,
        full_format: bool,
    },
    Literal {
        value: PrototypeLiteralValue,
        #[serde(default)]
        description: Option<String>,
    },
    Type {
        value: PrototypeType,
        description: String,
    },
    Struct,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum PrototypeLiteralValue {
    String(String),
    Number(f64),
    Bool(bool),
}
