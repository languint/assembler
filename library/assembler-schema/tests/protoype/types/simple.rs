#[cfg(test)]
mod simple {
    use crate::utils::de;
    use assembler_schema::schema::prelude::*;

    #[test]
    fn simple_named_type() {
        let PrototypeType::Simple(s) = de(r#""EquipmentID""#) else {
            panic!()
        };
        assert_eq!(s, "EquipmentID");
    }

    #[test]
    fn simple_builtin_string() {
        let t = de(r#""builtin""#);
        assert!(matches!(t, PrototypeType::Simple(_)));
    }
}
