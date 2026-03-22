#[cfg(test)]
mod concept {
    use crate::utils::de;
    use assembler_schema::schema::prelude::*;

    #[test]
    fn struct_concept_with_properties() {
        let c: PrototypeConcept = de(r#"{
            "name": "ActivateEquipmentCapsuleAction",
            "order": 0,
            "description": "",
            "abstract": false,
            "inline": false,
            "type": {"complex_type": "struct"},
            "properties": [{
                "name": "equipment",
                "order": 1,
                "description": "",
                "override": false,
                "type": "EquipmentID",
                "optional": false
            }]
        }"#);
        assert_eq!(c.basic_member.name, "ActivateEquipmentCapsuleAction");
        assert!(!c.r#abstract);
        assert!(!c.inline);
        let PrototypeType::Complex(inner) = &c.ty else {
            panic!()
        };
        assert!(matches!(**inner, PrototypeComplexType::Struct));
        assert_eq!(c.properties.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn builtin_concept_no_properties() {
        let c: PrototypeConcept = de(r#"{
            "name": "double",
            "order": 0,
            "description": "A double-precision float.",
            "abstract": false,
            "inline": false,
            "type": "builtin"
        }"#);
        assert!(matches!(c.ty, PrototypeType::Simple(_)));
        assert!(c.properties.is_none());
    }

    #[test]
    fn abstract_concept() {
        let c: PrototypeConcept = de(r#"{
            "name": "BaseStyleSpecification",
            "order": 0,
            "description": "",
            "abstract": true,
            "inline": false,
            "type": {"complex_type": "struct"}
        }"#);
        assert!(c.r#abstract);
    }

    #[test]
    fn inline_concept() {
        let c: PrototypeConcept = de(r#"{
            "name": "SomeInlineType",
            "order": 0,
            "description": "",
            "abstract": false,
            "inline": true,
            "type": "string"
        }"#);
        assert!(c.inline);
    }

    #[test]
    fn concept_with_parent() {
        let c: PrototypeConcept = de(r#"{
            "name": "ButtonStyleSpecification",
            "order": 0,
            "description": "",
            "parent": "StyleWithClickableGraphicSpecification",
            "abstract": false,
            "inline": false,
            "type": {"complex_type": "struct"}
        }"#);
        assert_eq!(
            c.parent.as_deref(),
            Some("StyleWithClickableGraphicSpecification")
        );
    }

    #[test]
    fn concept_properties_none_when_absent() {
        let c: PrototypeConcept = de(r#"{
            "name": "Foo",
            "order": 0,
            "description": "",
            "abstract": false,
            "inline": false,
            "type": "string"
        }"#);
        assert!(c.properties.is_none());
    }
}
