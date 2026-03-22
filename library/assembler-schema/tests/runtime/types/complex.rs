#[cfg(test)]
mod complex {
    use crate::utils;
    use assembler_schema::schema::prelude::*;

    fn de(s: &str) -> RuntimeType {
        utils::de(s)
    }

    fn unwrap_complex(t: RuntimeType) -> RuntimeComplexType {
        match t {
            RuntimeType::Complex(c) => *c,
            RuntimeType::Simple(s) => panic!("expected Complex, got Simple({s:?})"),
        }
    }

    #[test]
    fn complex_array_simple_value() {
        let c = unwrap_complex(de(
            r#"{"complex_type":"array","value":"LuaEntityPrototype"}"#,
        ));
        let RuntimeComplexType::Array { value } = c else {
            panic!()
        };
        assert!(matches!(value, RuntimeType::Simple(_)));
    }

    #[test]
    fn complex_array_nested_array() {
        let c = unwrap_complex(de(
            r#"{"complex_type":"array","value":{"complex_type":"array","value":"string"}}"#,
        ));
        let RuntimeComplexType::Array { value } = c else {
            panic!()
        };
        assert!(matches!(value, RuntimeType::Complex(_)));
    }

    #[test]
    fn complex_dictionary() {
        let c = unwrap_complex(de(
            r#"{"complex_type":"dictionary","key":"string","value":"string"}"#,
        ));
        let RuntimeComplexType::Dictionary { key, value } = c else {
            panic!()
        };
        assert!(matches!(key, RuntimeType::Simple(_)));
        assert!(matches!(value, RuntimeType::Simple(_)));
    }

    #[test]
    fn lua_custom_table_deserializes_as_dictionary() {
        let c = unwrap_complex(de(
            r#"{"complex_type":"LuaCustomTable","key":"string","value":"LuaRecipe"}"#,
        ));
        assert!(matches!(c, RuntimeComplexType::Dictionary { .. }));
    }

    #[test]
    fn complex_union() {
        let c = unwrap_complex(de(r#"{
        "complex_type": "union",
        "options": [
            {"complex_type": "literal", "value": "resource"},
            {"complex_type": "literal", "value": "terrain"},
            {"complex_type": "literal", "value": "cliff"},
            {"complex_type": "literal", "value": "enemy"}
        ],
        "full_format": false
    }"#));
        let RuntimeComplexType::Union {
            options,
            full_format,
        } = c
        else {
            panic!()
        };
        assert_eq!(options.len(), 4);
        assert!(!full_format);
    }

    #[test]
    fn complex_union_full_format_true() {
        let c = unwrap_complex(de(r#"{
        "complex_type": "union",
        "options": ["string", "uint"],
        "full_format": true
    }"#));
        let RuntimeComplexType::Union {
            full_format,
            options,
        } = c
        else {
            panic!()
        };
        assert!(full_format);
        assert_eq!(options.len(), 2);
    }

    #[test]
    fn complex_tuple() {
        let c = unwrap_complex(de(r#"{
        "complex_type": "tuple",
        "values": [
            "uint32",
            "defines.wire_connector_id",
            "uint32",
            "defines.wire_connector_id"
        ]
    }"#));
        let RuntimeComplexType::Tuple { values } = c else {
            panic!()
        };
        assert_eq!(values.len(), 4);
        assert!(values.iter().all(|v| matches!(v, RuntimeType::Simple(_))));
    }

    #[test]
    fn complex_function() {
        let c = unwrap_complex(de(
            r#"{"complex_type":"function","parameters":["EventData"]}"#,
        ));
        let RuntimeComplexType::Function { parameters } = c else {
            panic!()
        };
        assert_eq!(parameters.len(), 1);
    }

    #[test]
    fn complex_function_no_parameters() {
        let c = unwrap_complex(de(r#"{"complex_type":"function","parameters":[]}"#));
        let RuntimeComplexType::Function { parameters } = c else {
            panic!()
        };
        assert!(parameters.is_empty());
    }

    #[test]
    fn complex_table_minimal() {
        let c = unwrap_complex(de(r#"{
        "complex_type": "table",
        "parameters": [{
            "name": "expansion_shaders",
            "order": 0,
            "description": "",
            "type": "boolean",
            "optional": false
        }]
    }"#));
        let RuntimeComplexType::Table {
            parameters,
            variant_parameter_groups,
            variant_parameter_description,
        } = c
        else {
            panic!()
        };
        assert_eq!(parameters.len(), 1);
        assert!(variant_parameter_groups.is_none());
        assert!(variant_parameter_description.is_none());
    }

    #[test]
    fn complex_table_with_variant_groups() {
        let c = unwrap_complex(de(r#"{
        "complex_type": "table",
        "parameters": [
            {"name": "margin", "order": 0, "description": "", "type": "uint32",      "optional": false},
            {"name": "type",   "order": 1, "description": "", "type": "GuiArrowType","optional": false}
        ],
        "variant_parameter_groups": [{
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
        }],
        "variant_parameter_description": "Where to point."
    }"#));
        let RuntimeComplexType::Table {
            parameters,
            variant_parameter_groups,
            variant_parameter_description,
        } = c
        else {
            panic!()
        };
        assert_eq!(parameters.len(), 2);
        assert_eq!(variant_parameter_groups.as_ref().unwrap().len(), 1);
        assert!(variant_parameter_description.is_some());
    }

    #[test]
    fn complex_type_wrapper() {
        let c = unwrap_complex(de(r#"{
        "complex_type": "type",
        "value": "LuaAsteroidChunkPrototype",
        "description": "The asteroid chunk prototype."
    }"#));
        let RuntimeComplexType::Type { value, description } = c else {
            panic!()
        };
        assert!(matches!(value, RuntimeType::Simple(_)));
        assert!(!description.is_empty());
    }

    #[test]
    fn complex_literal_string() {
        let c = unwrap_complex(de(r#"{"complex_type":"literal","value":"resource"}"#));
        let RuntimeComplexType::Literal { value, description } = c else {
            panic!()
        };
        let RuntimeLiteralValue::String(s) = value else {
            panic!()
        };
        assert_eq!(s, "resource");
        assert!(description.is_none());
    }

    #[test]
    fn complex_literal_with_description() {
        let c = unwrap_complex(de(
            r#"{"complex_type":"literal","value":"foo","description":"some text"}"#,
        ));
        let RuntimeComplexType::Literal { description, .. } = c else {
            panic!()
        };
        assert_eq!(description.as_deref(), Some("some text"));
    }

    #[test]
    fn complex_lua_lazy_loaded_value() {
        let c = unwrap_complex(de(r#"{
        "complex_type": "LuaLazyLoadedValue",
        "value": {
            "complex_type": "dictionary",
            "key": "uint32",
            "value": "LuaEntity"
        }
    }"#));
        let RuntimeComplexType::LuaLazyLoadedValue { value } = c else {
            panic!()
        };
        assert!(matches!(value, RuntimeType::Complex(_)));
    }

    #[test]
    fn complex_lua_struct() {
        let c = unwrap_complex(de(r#"{
        "complex_type": "LuaStruct",
        "attributes": [{
            "name": "spoil_time_modifier",
            "order": 1,
            "description": "A value in range [0.01, 100].",
            "read_type": "double",
            "write_type": "double",
            "optional": false
        }]
    }"#));
        let RuntimeComplexType::LuaStruct { attributes } = c else {
            panic!()
        };
        assert_eq!(attributes.len(), 1);
        assert_eq!(attributes[0].basic_member.name, "spoil_time_modifier");
    }

    #[test]
    fn complex_builtin() {
        let c = unwrap_complex(de(r#"{"complex_type":"builtin"}"#));
        assert!(matches!(c, RuntimeComplexType::Builtin));
    }
}
