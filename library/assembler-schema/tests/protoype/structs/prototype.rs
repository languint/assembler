#[cfg(test)]
mod prototype {
    use crate::utils::de;
    use assembler_schema::schema::prelude::*;

    #[test]
    fn concrete_prototype_minimal() {
        let p: Prototype = de(r#"{
            "name": "AccumulatorPrototype",
            "order": 0,
            "description": "Entity with energy source.",
            "abstract": false,
            "deprecated": false,
            "typename": "accumulator",
            "properties": []
        }"#);
        assert_eq!(p.basic_member.name, "AccumulatorPrototype");
        assert!(!p.r#abstract);
        assert!(!p.deprecated);
        assert_eq!(p.typename.as_deref(), Some("accumulator"));
        assert!(p.parent.is_none());
        assert!(p.instance_limit.is_none());
        assert!(p.custom_properties.is_none());
    }

    #[test]
    fn abstract_prototype_has_no_typename() {
        let p: Prototype = de(r#"{
            "name": "EntityPrototype",
            "order": 0,
            "description": "",
            "abstract": true,
            "deprecated": false,
            "properties": []
        }"#);
        assert!(p.r#abstract);
        assert!(p.typename.is_none());
    }

    #[test]
    fn deprecated_prototype() {
        let p: Prototype = de(r#"{
            "name": "PlayerPortPrototype",
            "order": 0,
            "description": "",
            "abstract": false,
            "deprecated": true,
            "typename": "player-port",
            "properties": []
        }"#);
        assert!(p.deprecated);
    }

    #[test]
    fn prototype_with_instance_limit() {
        let p: Prototype = de(r#"{
            "name": "AmmoCategory",
            "order": 0,
            "description": "",
            "abstract": false,
            "deprecated": false,
            "typename": "ammo-category",
            "instance_limit": 255,
            "properties": []
        }"#);
        assert_eq!(p.instance_limit, Some(255));
    }

    #[test]
    fn prototype_with_parent() {
        let p: Prototype = de(r#"{
            "name": "BoilerPrototype",
            "order": 0,
            "description": "",
            "parent": "EntityWithHealthPrototype",
            "abstract": false,
            "deprecated": false,
            "typename": "boiler",
            "properties": []
        }"#);
        assert_eq!(p.parent.as_deref(), Some("EntityWithHealthPrototype"));
    }

    #[test]
    fn prototype_with_custom_properties() {
        let p: Prototype = de(r#"{
            "name": "GuiStyle",
            "order": 0,
            "description": "",
            "abstract": false,
            "deprecated": false,
            "properties": [],
            "custom_properties": {
                "description": "Styles are defined as uniquely named StyleSpecification properties.",
                "key_type": "string",
                "value_type": "StyleSpecification"
            }
        }"#);
        let cp = p.custom_properties.unwrap();
        assert!(!cp.description.is_empty());
    }

    #[test]
    fn prototype_with_visibility() {
        let p: Prototype = de(r#"{
            "name": "SpaceLocationPrototype",
            "order": 0,
            "description": "",
            "visibility": ["space_age"],
            "abstract": false,
            "deprecated": false,
            "typename": "space-location",
            "properties": []
        }"#);
        assert!(p.visibility.is_some());
    }

    #[test]
    fn prototype_with_properties() {
        let p: Prototype = de(r#"{
            "name": "SomePrototype",
            "order": 0,
            "description": "",
            "abstract": false,
            "deprecated": false,
            "properties": [
                {"name": "name",  "order": 0, "description": "", "override": false, "type": "string",  "optional": false},
                {"name": "icon",  "order": 1, "description": "", "override": false, "type": "FileName","optional": true}
            ]
        }"#);
        assert_eq!(p.properties.len(), 2);
    }
}
