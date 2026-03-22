#[cfg(test)]
mod class {
    use crate::utils::de;
    use assembler_schema::schema::prelude::*;

    #[test]
    fn minimal_concrete_class() {
        let c: Class = de(r#"{
            "name": "LuaAISettings",
            "order": 0,
            "description": "Collection of settings for overriding default ai behavior.",
            "abstract": false,
            "methods": [],
            "attributes": [{
                "name": "allow_destroy_when_commands_fail",
                "order": 0,
                "description": "",
                "read_type": "boolean",
                "write_type": "boolean",
                "optional": false
            }],
            "operators": []
        }"#);
        assert_eq!(c.basic_member.name, "LuaAISettings");
        assert!(!c.r#abstract);
        assert!(c.parent.is_none());
        assert_eq!(c.attributes.len(), 1);
        assert!(c.methods.is_empty());
        assert!(c.operators.is_empty());
    }

    #[test]
    fn class_with_parent() {
        let c: Class = de(r#"{
            "name": "LuaAccumulatorControlBehavior",
            "order": 1,
            "description": "",
            "abstract": false,
            "parent": "LuaControlBehavior",
            "methods": [],
            "attributes": [],
            "operators": []
        }"#);
        assert_eq!(c.parent.as_deref(), Some("LuaControlBehavior"));
    }

    #[test]
    fn abstract_class() {
        let c: Class = de(r#"{
            "name": "LuaControl",
            "order": 0,
            "description": "",
            "abstract": true,
            "methods": [],
            "attributes": [],
            "operators": []
        }"#);
        assert!(c.r#abstract);
    }

    #[test]
    fn class_with_call_operator() {
        let c: Class = de(r#"{
            "name": "LuaChunkIterator",
            "order": 0,
            "description": "",
            "abstract": false,
            "methods": [],
            "attributes": [],
            "operators": [{
                "name": "call",
                "order": 0,
                "description": "",
                "parameters": [],
                "format": {"takes_table": false},
                "return_values": []
            }]
        }"#);
        assert_eq!(c.operators.len(), 1);
    }

    #[test]
    fn class_with_visibility() {
        let c: Class = de(r#"{
            "name": "LuaSpaceAge",
            "order": 0,
            "description": "",
            "visibility": ["space_age"],
            "abstract": false,
            "methods": [],
            "attributes": [],
            "operators": []
        }"#);
        assert!(c.visibility.is_some());
    }
}
