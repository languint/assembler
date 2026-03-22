#[cfg(test)]
mod simple {
    use crate::utils::de;
    use assembler_schema::schema::prelude::*;

    #[test]
    fn simple_named_type() {
        let RuntimeType::Simple(s) = de(r#""LuaEntity""#) else {
            panic!()
        };
        assert_eq!(s, "LuaEntity");
    }

    #[test]
    fn simple_builtin_primitives() {
        for name in &[
            "string", "boolean", "uint", "uint8", "uint16", "uint32", "uint64", "int", "int8",
            "double", "float", "nil", "Any",
        ] {
            assert!(
                matches!(de(&format!(r#""{name}""#)), RuntimeType::Simple(_)),
                "failed for {name}"
            );
        }
    }
}
