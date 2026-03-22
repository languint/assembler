#[cfg(test)]
mod literal_value {
    use crate::utils::de;
    use assembler_schema::schema::prelude::*;

    #[test]
    fn string_variant() {
        let RuntimeLiteralValue::String(s) = de(r#""resource""#) else {
            panic!()
        };
        assert_eq!(s, "resource");
    }

    #[test]
    fn number_variant() {
        let v: RuntimeLiteralValue = de(r#"42.5"#);
        assert!(matches!(v, RuntimeLiteralValue::Number(_)));
    }

    #[test]
    fn bool_true() {
        let v: RuntimeLiteralValue = de(r#"true"#);
        assert!(matches!(v, RuntimeLiteralValue::Bool(true)));
    }

    #[test]
    fn bool_false() {
        let v: RuntimeLiteralValue = de(r#"false"#);
        assert!(matches!(v, RuntimeLiteralValue::Bool(false)));
    }

    #[test]
    fn integer_zero_is_number_not_bool() {
        let v: RuntimeLiteralValue = de(r#"0"#);
        assert!(matches!(v, RuntimeLiteralValue::Number(_)));
    }
}
