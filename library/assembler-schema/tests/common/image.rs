#[cfg(test)]
mod image {
    use crate::utils::de;
    use assembler_schema::prelude::*;

    #[test]
    fn without_caption() {
        let img: Image = de(r#"{"filename":"foo.png"}"#);
        assert_eq!(img.filename, "foo.png");
        assert!(img.caption.is_none());
    }

    #[test]
    fn with_caption() {
        let img: Image = de(r#"{"filename":"bar.png","caption":"An explanation."}"#);
        assert_eq!(img.caption.as_deref(), Some("An explanation."));
    }
}
