use crate::schema::prelude::*;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "name", rename_all = "lowercase")]
pub enum Operator {
    Call(Method),
    Index(Attribute),
    Length(Attribute),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_call() {
        let json = r#"{
            "name": "call",
            "order": 0,
            "description": "",
            "lists": [],
            "examples": [],
            "images": [],
            "visibility": null,
            "raises": null,
            "subclasses": null,
            "parameters": [],
            "variant_parameter_groups": null,
            "variant_parameter_description": null,
            "variadic_parameter": null,
            "format": { "takes_table": false, "table_optional": null },
            "return_values": []
        }"#;
        let t: Operator = serde_json::from_str(json).unwrap();
        assert!(matches!(t, Operator::Call(_)));
    }

    #[test]
    fn operator_index() {
        let json = r#"{
            "name": "index",
            "order": 0,
            "description": "",
            "lists": [],
            "examples": [],
            "images": [],
            "visibility": null,
            "raises": null,
            "subclasses": null,
            "read_type": null,
            "write_type": null,
            "optional": false
        }"#;
        let t: Operator = serde_json::from_str(json).unwrap();
        assert!(matches!(t, Operator::Index(_)));
    }
}
