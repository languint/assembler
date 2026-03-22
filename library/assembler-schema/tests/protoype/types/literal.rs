#[cfg(test)]
mod literal {
    use crate::utils::de;
    use assembler_schema::prelude::*;

    #[test]
    fn string_variant() {
        let v: PrototypeLiteralValue = de(r#""hello""#);
        assert!(matches!(v, PrototypeLiteralValue::String(_)));
    }

    #[test]
    fn number_variant() {
        let v: PrototypeLiteralValue = de(r#"3.14"#);
        assert!(matches!(v, PrototypeLiteralValue::Number(_)));
    }

    #[test]
    fn bool_variant() {
        let v: PrototypeLiteralValue = de(r#"true"#);
        assert!(matches!(v, PrototypeLiteralValue::Bool(true)));
    }

    #[test]
    fn zero_is_number_not_bool() {
        let v: PrototypeLiteralValue = de(r#"0"#);
        assert!(matches!(v, PrototypeLiteralValue::Number(_)));
    }
}
