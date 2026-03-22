#[cfg(test)]
mod complex {
    use crate::utils::de;
    use assembler_schema::schema::prelude::*;

    fn unwrap_complex(t: PrototypeType) -> PrototypeComplexType {
        match t {
            PrototypeType::Complex(c) => *c,
            PrototypeType::Simple(s) => panic!("expected Complex, got Simple({s:?})"),
        }
    }

    #[test]
    fn complex_array() {
        let c = unwrap_complex(de(r#"{"complex_type":"array","value":"string"}"#));
        let PrototypeComplexType::Array { value } = c else {
            panic!()
        };
        assert!(matches!(value, PrototypeType::Simple(_)));
    }

    #[test]
    fn complex_array_of_complex() {
        let c = unwrap_complex(de(
            r#"{"complex_type":"array","value":{"complex_type":"array","value":"uint"}}"#,
        ));
        let PrototypeComplexType::Array { value } = c else {
            panic!()
        };
        assert!(matches!(value, PrototypeType::Complex(_)));
    }

    #[test]
    fn complex_dictionary() {
        let c = unwrap_complex(de(
            r#"{"complex_type":"dictionary","key":"string","value":"StyleSpecification"}"#,
        ));
        let PrototypeComplexType::Dictionary { key, value } = c else {
            panic!()
        };
        assert!(matches!(key, PrototypeType::Simple(_)));
        assert!(matches!(value, PrototypeType::Simple(_)));
    }

    #[test]
    fn complex_tuple() {
        let c = unwrap_complex(de(
            r#"{"complex_type":"tuple","values":["double","double"]}"#,
        ));
        let PrototypeComplexType::Tuple { values } = c else {
            panic!()
        };
        assert_eq!(values.len(), 2);
    }

    #[test]
    fn complex_tuple_heterogeneous() {
        let c = unwrap_complex(de(
            r#"{"complex_type":"tuple","values":["uint8","uint8","uint8","uint8"]}"#,
        ));
        let PrototypeComplexType::Tuple { values } = c else {
            panic!()
        };
        assert_eq!(values.len(), 4);
    }

    #[test]
    fn complex_union() {
        let c = unwrap_complex(de(r#"{
           "complex_type": "union",
           "options": ["Energy", "double"],
           "full_format": false
       }"#));
        let PrototypeComplexType::Union {
            options,
            full_format,
        } = c
        else {
            panic!()
        };
        assert_eq!(options.len(), 2);
        assert!(!full_format);
    }

    #[test]
    fn complex_union_full_format() {
        let c = unwrap_complex(de(r#"{
           "complex_type": "union",
           "options": ["string", "uint"],
           "full_format": true
       }"#));
        let PrototypeComplexType::Union { full_format, .. } = c else {
            panic!()
        };
        assert!(full_format);
    }

    #[test]
    fn complex_literal_string() {
        let c = unwrap_complex(de(
            r#"{"complex_type":"literal","value":"equipment-remote"}"#,
        ));
        let PrototypeComplexType::Literal { value, description } = c else {
            panic!()
        };
        let PrototypeLiteralValue::String(s) = value else {
            panic!()
        };
        assert_eq!(s, "equipment-remote");
        assert!(description.is_none());
    }

    #[test]
    fn complex_literal_number_zero() {
        let c = unwrap_complex(de(r#"{"complex_type":"literal","value":0}"#));
        let PrototypeComplexType::Literal { value, .. } = c else {
            panic!()
        };
        let PrototypeLiteralValue::Number(n) = value else {
            panic!()
        };
        assert_eq!(n, 0.0);
    }

    #[test]
    fn complex_literal_number_nonzero() {
        let c = unwrap_complex(de(r#"{"complex_type":"literal","value":255}"#));
        let PrototypeComplexType::Literal { value, .. } = c else {
            panic!()
        };
        assert!(matches!(value, PrototypeLiteralValue::Number(_)));
    }

    #[test]
    fn complex_literal_bool_true() {
        let c = unwrap_complex(de(r#"{"complex_type":"literal","value":true}"#));
        let PrototypeComplexType::Literal { value, .. } = c else {
            panic!()
        };
        assert!(matches!(value, PrototypeLiteralValue::Bool(true)));
    }

    #[test]
    fn complex_literal_bool_false() {
        let c = unwrap_complex(de(r#"{"complex_type":"literal","value":false}"#));
        let PrototypeComplexType::Literal { value, .. } = c else {
            panic!()
        };
        assert!(matches!(value, PrototypeLiteralValue::Bool(false)));
    }

    #[test]
    fn complex_literal_with_description() {
        let c = unwrap_complex(de(
            r#"{"complex_type":"literal","value":"foo","description":"Some meaning."}"#,
        ));
        let PrototypeComplexType::Literal { description, .. } = c else {
            panic!()
        };
        assert_eq!(description.as_deref(), Some("Some meaning."));
    }

    #[test]
    fn complex_type_wrapper() {
        let c = unwrap_complex(de(
            r#"{"complex_type":"type","value":"string","description":"A name."}"#,
        ));
        let PrototypeComplexType::Type { value, description } = c else {
            panic!()
        };
        assert!(matches!(value, PrototypeType::Simple(_)));
        assert_eq!(description, "A name.");
    }

    #[test]
    fn complex_type_wrapper_nested() {
        let c = unwrap_complex(de(r#"{
           "complex_type": "type",
           "value": {"complex_type": "array", "value": "string"},
           "description": "A list of names."
       }"#));
        let PrototypeComplexType::Type { value, .. } = c else {
            panic!()
        };
        assert!(matches!(value, PrototypeType::Complex(_)));
    }

    #[test]
    fn complex_struct_no_extra_fields() {
        let c = unwrap_complex(de(r#"{"complex_type":"struct"}"#));
        assert!(matches!(c, PrototypeComplexType::Struct));
    }
}
