#[cfg(test)]
mod stage {
    use crate::utils::de;
    use assembler_schema::prelude::*;

    #[test]
    fn runtime_variant() {
        let s: Stage = de(r#""runtime""#);
        assert!(matches!(s, Stage::Runtime));
    }

    #[test]
    fn prototype_variant() {
        let s: Stage = de(r#""prototype""#);
        assert!(matches!(s, Stage::Prototype));
    }
}
