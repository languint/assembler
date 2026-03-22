#[cfg(test)]
mod visibility {
    use crate::utils::{assert_de_fails, de};
    use assembler_schema::schema::prelude::*;

    #[test]
    fn space_age_variant() {
        let v: Visibility = de(r#""space_age""#);
        assert!(matches!(v, Visibility::SpaceAge));
    }

    #[test]
    fn unknown_variant_errors() {
        assert_de_fails::<Visibility>(r#""core""#);
    }

    #[test]
    fn in_array() {
        let v: Vec<Visibility> = de(r#"["space_age"]"#);
        assert_eq!(v.len(), 1);
        assert!(matches!(v[0], Visibility::SpaceAge));
    }
}
