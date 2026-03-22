#[cfg(test)]
mod method {
    use crate::utils::de;
    use assembler_schema::schema::prelude::*;

    #[test]
    fn minimal_method() {
        let m: Method = de(r#"{
            "name": "destroy",
            "order": 0,
            "description": "Destroys this entity.",
            "parameters": [],
            "format": {"takes_table": false},
            "return_values": []
        }"#);
        assert_eq!(m.basic_member.name, "destroy");
        assert!(!m.format.takes_table);
        assert!(m.format.table_optional.is_none());
        assert!(m.raises.is_none());
        assert!(m.variadic_parameter.is_none());
        assert!(m.subclasses.is_none());
    }

    #[test]
    fn takes_table_with_explicit_optional_false() {
        let m: Method = de(r#"{
            "name": "raise_biter_base_built",
            "order": 17,
            "description": "",
            "raises": [{
                "name": "on_biter_base_built",
                "order": 0,
                "description": "Raised with the provided arguments.",
                "timeframe": "instantly",
                "optional": false
            }],
            "parameters": [{
                "name": "entity",
                "order": 0,
                "description": "The entity that was built.",
                "type": "LuaEntity",
                "optional": false
            }],
            "format": {"takes_table": true, "table_optional": false},
            "return_values": []
        }"#);
        assert!(m.format.takes_table);
        assert_eq!(m.format.table_optional, Some(false));
        assert_eq!(m.raises.as_ref().unwrap().len(), 1);
        assert_eq!(m.parameters.len(), 1);
    }

    #[test]
    fn method_with_variadic_parameter() {
        let m: Method = de(r#"{
            "name": "call",
            "order": 0,
            "description": "",
            "parameters": [
                {"name": "interface", "order": 0, "description": "", "type": "string", "optional": false},
                {"name": "function",  "order": 1, "description": "", "type": "string", "optional": false}
            ],
            "variadic_parameter": {
                "type": "Any",
                "description": "Arguments to pass to the called function."
            },
            "format": {"takes_table": false},
            "return_values": []
        }"#);
        let vp = m.variadic_parameter.unwrap();
        assert!(vp.ty.is_some());
        assert!(vp.description.is_some());
    }

    #[test]
    fn variadic_parameter_all_fields_optional() {
        let m: Method = de(r#"{
            "name": "log",
            "order": 0,
            "description": "",
            "parameters": [],
            "variadic_parameter": {},
            "format": {"takes_table": false},
            "return_values": []
        }"#);
        let vp = m.variadic_parameter.unwrap();
        assert!(vp.ty.is_none());
        assert!(vp.description.is_none());
    }

    #[test]
    fn method_with_subclasses() {
        let m: Method = de(r#"{
            "name": "get_unit_group",
            "order": 0,
            "description": "",
            "subclasses": ["UnitGroup"],
            "parameters": [],
            "format": {"takes_table": false},
            "return_values": []
        }"#);
        assert_eq!(m.subclasses.as_ref().unwrap(), &["UnitGroup"]);
    }

    #[test]
    fn method_multiple_return_values() {
        let m: Method = de(r#"{
            "name": "get_position",
            "order": 0,
            "description": "",
            "parameters": [],
            "format": {"takes_table": false},
            "return_values": [
                {"name": "", "order": 0, "description": "", "type": "double", "optional": false},
                {"name": "", "order": 1, "description": "", "type": "double", "optional": false}
            ]
        }"#);
        assert_eq!(m.return_values.len(), 2);
    }

    #[test]
    fn method_optional_return_value() {
        let m: Method = de(r#"{
            "name": "get_player",
            "order": 0,
            "description": "",
            "parameters": [{"name":"p","order":0,"description":"","type":"uint32","optional":false}],
            "format": {"takes_table": false},
            "return_values": [{"name": "", "order": 0, "description": "", "type": "LuaPlayer", "optional": true}]
        }"#);
        assert!(m.return_values[0].optional);
    }

    #[test]
    fn method_with_variant_parameter_groups() {
        let m: Method = de(r#"{
            "name": "set_gui_arrow",
            "order": 6,
            "description": "",
            "parameters": [
                {"name": "margin", "order": 0, "description": "", "type": "uint32",       "optional": false},
                {"name": "type",   "order": 1, "description": "", "type": "GuiArrowType", "optional": false}
            ],
            "variant_parameter_groups": [{
                "name": "entity",
                "order": 0,
                "description": "",
                "parameters": [{"name":"entity","order":0,"description":"","type":"LuaEntity","optional":false}]
            }],
            "variant_parameter_description": "Where to point.",
            "format": {"takes_table": true, "table_optional": false},
            "return_values": []
        }"#);
        assert_eq!(m.variant_parameter_groups.as_ref().unwrap().len(), 1);
        assert_eq!(
            m.variant_parameter_description.as_deref(),
            Some("Where to point.")
        );
    }

    #[test]
    fn method_with_visibility() {
        let m: Method = de(r#"{
            "name": "space_method",
            "order": 0,
            "description": "",
            "visibility": ["space_age"],
            "parameters": [],
            "format": {"takes_table": false},
            "return_values": []
        }"#);
        assert!(m.visibility.is_some());
    }
}
