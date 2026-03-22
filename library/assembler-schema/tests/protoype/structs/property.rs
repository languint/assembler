#[cfg(test)]
mod property {
    use crate::utils::de;
    use assembler_schema::schema::prelude::*;

    #[test]
    fn minimal_property() {
        let p: Property = de(r#"{
            "name": "equipment",
            "order": 1,
            "description": "Activation is only implemented for ActiveDefenseEquipmentPrototype.",
            "override": false,
            "type": "EquipmentID",
            "optional": false
        }"#);
        assert_eq!(p.basic_member.name, "equipment");
        assert!(!p.r#override);
        assert!(!p.optional);
        assert!(p.default.is_none());
        assert!(p.alt_name.is_none());
        assert!(p.visibility.is_none());
    }

    #[test]
    fn property_with_literal_default() {
        let p: Property = de(r#"{
            "name": "circuit_wire_max_distance",
            "order": 2,
            "description": "The maximum circuit wire distance for this entity.",
            "override": false,
            "type": "double",
            "optional": true,
            "default": {"complex_type": "literal", "value": 0}
        }"#);
        assert!(p.optional);
        let Some(PropertyDefault::Literal(ty)) = p.default else {
            panic!()
        };
        let PrototypeType::Complex(inner) = ty else {
            panic!()
        };
        let PrototypeComplexType::Literal { value, .. } = *inner else {
            panic!()
        };
        assert!(matches!(value, PrototypeLiteralValue::Number(_)));
    }

    #[test]
    fn property_with_text_default() {
        let p: Property = de(r#"{
            "name": "energy_usage",
            "order": 0,
            "description": "",
            "override": false,
            "type": "Energy",
            "optional": true,
            "default": "0J if not set"
        }"#);
        let Some(PropertyDefault::Text(s)) = p.default else {
            panic!()
        };
        assert_eq!(s, "0J if not set");
    }

    #[test]
    fn property_with_alt_name() {
        let p: Property = de(r#"{
            "name": "braking_power",
            "order": 1,
            "description": "",
            "alt_name": "braking_force",
            "override": false,
            "type": "double",
            "optional": false
        }"#);
        assert_eq!(p.alt_name.as_deref(), Some("braking_force"));
    }

    #[test]
    fn property_override_true() {
        let p: Property = de(r#"{
            "name": "icon",
            "order": 0,
            "description": "",
            "override": true,
            "type": "FileName",
            "optional": true
        }"#);
        assert!(p.r#override);
    }

    #[test]
    fn property_with_visibility() {
        let p: Property = de(r#"{
            "name": "surface_conditions",
            "order": 0,
            "description": "",
            "visibility": ["space_age"],
            "override": false,
            "type": "string",
            "optional": true
        }"#);
        assert!(p.visibility.is_some());
    }

    #[test]
    fn property_with_complex_type() {
        let p: Property = de(r#"{
            "name": "flags",
            "order": 0,
            "description": "",
            "override": false,
            "type": {"complex_type": "array", "value": "string"},
            "optional": false
        }"#);
        assert!(matches!(p.ty, PrototypeType::Complex(_)));
    }

    #[test]
    fn property_bool_literal_default() {
        let p: Property = de(r#"{
            "name": "enabled",
            "order": 0,
            "description": "",
            "override": false,
            "type": "boolean",
            "optional": true,
            "default": {"complex_type": "literal", "value": true}
        }"#);
        let Some(PropertyDefault::Literal(ty)) = p.default else {
            panic!()
        };
        let PrototypeType::Complex(inner) = ty else {
            panic!()
        };
        let PrototypeComplexType::Literal { value, .. } = *inner else {
            panic!()
        };
        assert!(matches!(value, PrototypeLiteralValue::Bool(true)));
    }
}
