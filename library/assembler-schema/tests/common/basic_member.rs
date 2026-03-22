#[cfg(test)]
mod basic_member {
    use crate::utils::de;
    use assembler_schema::schema::prelude::*;

    #[test]
    fn minimal_required_fields() {
        let m: BasicMember = de(r#"{"name":"foo","order":0,"description":""}"#);
        assert_eq!(m.name, "foo");
        assert_eq!(m.order, 0);
        assert_eq!(m.description, "");
        assert!(m.lists.is_none());
        assert!(m.examples.is_none());
        assert!(m.images.is_none());
    }

    #[test]
    fn description_can_be_empty_string() {
        let m: BasicMember = de(r#"{"name":"x","order":0,"description":""}"#);
        assert_eq!(m.description, "");
    }

    #[test]
    fn all_optional_fields_present() {
        let m: BasicMember = de(r#"{
            "name": "bar",
            "order": 3,
            "description": "A description.",
            "lists": ["- item one\n- item two"],
            "examples": ["```\nfoo()\n```"],
            "images": [{"filename": "foo.png"}]
        }"#);
        assert_eq!(m.order, 3);
        assert_eq!(m.lists.as_ref().unwrap().len(), 1);
        assert_eq!(m.examples.as_ref().unwrap().len(), 1);
        assert_eq!(m.images.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn high_order_value() {
        let m: BasicMember = de(r#"{"name":"x","order":9999,"description":""}"#);
        assert_eq!(m.order, 9999);
    }
}
