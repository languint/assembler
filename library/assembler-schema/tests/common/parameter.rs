#[cfg(test)]
mod parameter {
    use crate::utils::de;
    use assembler_schema::prelude::*;

    #[test]
    fn required_parameter_with_simple_type() {
        let p: Parameter = de(r#"{
            "name": "entity",
            "order": 0,
            "description": "The entity that was built.",
            "type": "LuaEntity",
            "optional": false
        }"#);
        assert_eq!(p.name, "entity");
        assert!(!p.optional);
        assert!(matches!(p.ty, RuntimeType::Simple(_)));
    }

    #[test]
    fn optional_parameter() {
        let p: Parameter = de(r#"{
            "name": "result",
            "order": 0,
            "description": "",
            "type": "ChunkPositionAndArea",
            "optional": true
        }"#);
        assert!(p.optional);
    }

    #[test]
    fn parameter_with_complex_type() {
        let p: Parameter = de(r#"{
            "name": "items",
            "order": 0,
            "description": "",
            "type": {"complex_type": "array", "value": "string"},
            "optional": false
        }"#);
        assert!(matches!(p.ty, RuntimeType::Complex(_)));
    }

    #[test]
    fn parameter_group_with_members() {
        let g: ParameterGroup = de(r#"{
            "name": "entity",
            "order": 0,
            "description": "",
            "parameters": [{
                "name": "entity",
                "order": 0,
                "description": "",
                "type": "LuaEntity",
                "optional": false
            }]
        }"#);
        assert_eq!(g.name, "entity");
        assert_eq!(g.parameters.len(), 1);
    }

    #[test]
    fn parameter_group_can_be_empty() {
        let g: ParameterGroup = de(r#"{"name":"none","order":0,"description":"","parameters":[]}"#);
        assert!(g.parameters.is_empty());
    }
}
