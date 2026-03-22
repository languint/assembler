#[cfg(test)]
mod attribute {
    use crate::utils::de;
    use assembler_schema::schema::prelude::*;

    #[test]
    fn read_write_attribute() {
        let a: Attribute = de(r#"{
           "name": "allow_destroy_when_commands_fail",
           "order": 0,
           "description": "If enabled, units that repeatedly fail to succeed at commands will be destroyed.",
           "read_type": "boolean",
           "write_type": "boolean",
           "optional": false
       }"#);
        assert_eq!(a.basic_member.name, "allow_destroy_when_commands_fail");
        assert!(a.read_type.is_some());
        assert!(a.write_type.is_some());
        assert!(!a.optional);
    }

    #[test]
    fn read_only_attribute() {
        let a: Attribute = de(r#"{
           "name": "object_name",
           "order": 7,
           "description": "The class name of this object.",
           "read_type": "string",
           "optional": false
       }"#);
        assert!(a.read_type.is_some());
        assert!(a.write_type.is_none());
    }

    #[test]
    fn write_only_attribute() {
        let a: Attribute = de(r#"{
           "name": "sink",
           "order": 0,
           "description": "",
           "write_type": "string",
           "optional": false
       }"#);
        assert!(a.read_type.is_none());
        assert!(a.write_type.is_some());
    }

    #[test]
    fn optional_attribute() {
        let a: Attribute = de(r#"{
           "name": "index",
           "order": 0,
           "description": "The indexing operator. Gets children by name.",
           "read_type": "LuaGuiElement",
           "optional": true
       }"#);
        assert!(a.optional);
    }

    #[test]
    fn attribute_with_raises() {
        let a: Attribute = de(r#"{
           "name": "active",
           "order": 0,
           "description": "",
           "raises": [{
               "name": "on_entity_settings_pasted",
               "order": 0,
               "description": "",
               "timeframe": "current_tick",
               "optional": true
           }],
           "read_type": "boolean",
           "write_type": "boolean",
           "optional": false
       }"#);
        assert_eq!(a.raises.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn attribute_with_subclasses() {
        let a: Attribute = de(r#"{
           "name": "members",
           "order": 0,
           "description": "",
           "subclasses": ["UnitGroup", "SpiderVehicle"],
           "read_type": "boolean",
           "optional": false
       }"#);
        assert_eq!(a.subclasses.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn attribute_with_visibility() {
        let a: Attribute = de(r#"{
           "name": "surface_conditions",
           "order": 0,
           "description": "",
           "visibility": ["space_age"],
           "read_type": "string",
           "optional": true
       }"#);
        assert!(a.visibility.is_some());
    }
}
