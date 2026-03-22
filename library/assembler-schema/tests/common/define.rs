#[cfg(test)]
mod define {
    use crate::utils::de;
    use assembler_schema::schema::prelude::*;

    #[test]
    fn flat_define_with_values() {
        let d: Define = de(r#"{
            "name": "alert_type",
            "order": 0,
            "description": "",
            "values": [
                {"name": "custom",           "order": 1, "description": ""},
                {"name": "entity_destroyed", "order": 2, "description": ""}
            ]
        }"#);
        assert_eq!(d.basic_member.name, "alert_type");
        assert_eq!(d.values.as_ref().unwrap().len(), 2);
        assert!(d.subkeys.is_none());
    }

    #[test]
    fn define_with_no_values_and_no_subkeys() {
        let d: Define = de(r#"{"name":"empty","order":0,"description":""}"#);
        assert!(d.values.is_none());
        assert!(d.subkeys.is_none());
    }

    #[test]
    fn recursive_define_two_levels_deep() {
        let d: Define = de(r#"{
            "name": "control_behavior",
            "order": 9,
            "description": "",
            "subkeys": [{
                "name": "inserter",
                "order": 0,
                "description": "",
                "subkeys": [{
                    "name": "circuit_mode_of_operation",
                    "order": 0,
                    "description": "",
                    "values": [
                        {"name": "enable_disable", "order": 0, "description": ""},
                        {"name": "set_filters",    "order": 1, "description": ""}
                    ]
                }]
            }]
        }"#);
        let l1 = d.subkeys.as_ref().unwrap();
        assert_eq!(l1.len(), 1);
        let l2 = l1[0].subkeys.as_ref().unwrap();
        assert_eq!(l2[0].basic_member.name, "circuit_mode_of_operation");
        assert_eq!(l2[0].values.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn define_value_fields() {
        let v: DefineValue = de(r#"{"name":"none","order":0,"description":""}"#);
        assert_eq!(v.name, "none");
        assert_eq!(v.order, 0);
        assert_eq!(v.description, "");
    }

    #[test]
    fn define_value_does_not_include_extra_fields() {
        let v: DefineValue = de(r#"{"name":"x","order":5,"description":"desc"}"#);
        assert_eq!(v.name, "x");
        assert_eq!(v.order, 5);
    }
}
