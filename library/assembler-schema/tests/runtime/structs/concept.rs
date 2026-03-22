#[cfg(test)]
mod concept {
    use crate::utils::de;
    use assembler_schema::schema::prelude::*;

    #[test]
    fn concept_with_simple_type() {
        let c: RuntimeConcept = de(r#"{
            "name": "PlayerIndex",
            "order": 0,
            "description": "A player index.",
            "type": "uint"
        }"#);
        assert!(matches!(c.ty, RuntimeType::Simple(_)));
    }

    #[test]
    fn concept_with_builtin_type() {
        let c: RuntimeConcept = de(r#"{
            "name": "LuaObject",
            "order": 372,
            "description": "Any LuaObject listed on the Classes page.",
            "type": {"complex_type": "builtin"}
        }"#);
        let RuntimeType::Complex(inner) = c.ty else {
            panic!()
        };
        assert!(matches!(*inner, RuntimeComplexType::Builtin));
    }

    #[test]
    fn concept_with_table_type() {
        let c: RuntimeConcept = de(r#"{
            "name": "AccumulatorBlueprintControlBehavior",
            "order": 180,
            "description": "",
            "type": {
                "complex_type": "table",
                "parameters": [{
                    "name": "output_signal",
                    "order": 0,
                    "description": "",
                    "type": "SignalID",
                    "optional": false
                }]
            }
        }"#);
        let RuntimeType::Complex(inner) = c.ty else {
            panic!()
        };
        assert!(matches!(*inner, RuntimeComplexType::Table { .. }));
    }

    #[test]
    fn concept_with_union_type() {
        let c: RuntimeConcept = de(r#"{
            "name": "MapGenSize",
            "order": 0,
            "description": "",
            "type": {
                "complex_type": "union",
                "options": ["float", "string"],
                "full_format": false
            }
        }"#);
        let RuntimeType::Complex(inner) = c.ty else {
            panic!()
        };
        assert!(matches!(*inner, RuntimeComplexType::Union { .. }));
    }
}
