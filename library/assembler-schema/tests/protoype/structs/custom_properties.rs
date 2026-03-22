#[cfg(test)]
mod custom_properties {
    use crate::utils::de;
    use assembler_schema::prelude::*;

    #[test]
    fn minimal() {
        let cp: CustomProperties = de(r#"{
            "description": "Custom style properties.",
            "key_type": "string",
            "value_type": "StyleSpecification"
        }"#);
        assert_eq!(cp.description, "Custom style properties.");
        assert!(cp.lists.is_none());
        assert!(cp.examples.is_none());
        assert!(cp.images.is_none());
    }

    #[test]
    fn with_all_optional_fields() {
        let cp: CustomProperties = de(r#"{
            "description": "desc",
            "lists": ["- item"],
            "examples": ["```\nfoo\n```"],
            "images": [{"filename": "foo.png", "caption": "caption"}],
            "key_type": "string",
            "value_type": "StyleSpecification"
        }"#);
        assert_eq!(cp.lists.as_ref().unwrap().len(), 1);
        assert_eq!(cp.examples.as_ref().unwrap().len(), 1);
        assert_eq!(cp.images.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn complex_key_and_value_types() {
        let cp: CustomProperties = de(r#"{
            "description": "",
            "key_type": {"complex_type": "union", "options": ["string", "uint"], "full_format": false},
            "value_type": {"complex_type": "array", "value": "string"}
        }"#);

        assert!(matches!(cp.key_type, PrototypeType::Complex(_)));
        assert!(matches!(cp.value_type, PrototypeType::Complex(_)));
    }
}
