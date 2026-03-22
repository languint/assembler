#[cfg(test)]
mod full_parses {
    use crate::utils::{prototype_json, runtime_json};
    use assembler_schema::schema::prelude::*;

    #[test]
    fn parses_full_runtime_api() {
        let Some(json) = runtime_json() else {
            eprintln!("SKIPPED: FACTORIO_HOME not set");
            return;
        };

        let root: RuntimeApiRoot =
            serde_json::from_str(&json).expect("runtime-api.json should deserialize without error");

        assert_eq!(root.application, "factorio");
        assert_eq!(root.stage, Stage::Runtime);
        assert_eq!(root.api_version, 6);

        // This needs to be updated when factorio updates
        assert_eq!(root.classes.len(), 148, "class count changed");
        assert_eq!(root.events.len(), 219, "event count changed");
        assert_eq!(root.concepts.len(), 418, "concept count changed");
        assert_eq!(root.defines.len(), 60, "define count changed");
        assert_eq!(root.global_objects.len(), 9, "global_objects count changed");
        assert_eq!(
            root.global_functions.len(),
            3,
            "global_functions count changed"
        );
    }

    #[test]
    fn parses_full_prototype_api() {
        let Some(json) = prototype_json() else {
            eprintln!("SKIPPED: FACTORIO_HOME not set");
            return;
        };

        let root: PrototypeApiRoot = serde_json::from_str(&json)
            .expect("prototype-api.json should deserialize without error");

        assert_eq!(root.application, "factorio");
        assert_eq!(root.api_version, 6);

        assert_eq!(root.prototypes.len(), 278, "prototype count changed");
        assert_eq!(root.types.len(), 686, "type count changed");
        assert_eq!(root.defines.len(), 60, "define count changed");
    }

    #[test]
    fn runtime_lua_ai_settings_class() {
        let Some(json) = runtime_json() else {
            return;
        };
        let root: RuntimeApiRoot = serde_json::from_str(&json).unwrap();

        let cls = root
            .classes
            .iter()
            .find(|c| c.basic_member.name == "LuaAISettings")
            .expect("LuaAISettings should exist");

        assert!(!cls.r#abstract);
        assert!(cls.parent.is_none());
        assert!(cls.methods.is_empty());
        assert!(!cls.attributes.is_empty());

        let attr = cls
            .attributes
            .iter()
            .find(|a| a.basic_member.name == "allow_destroy_when_commands_fail")
            .expect("attribute should exist");
        assert!(attr.read_type.is_some());
        assert!(attr.write_type.is_some());
        assert!(!attr.optional);
    }

    #[test]
    fn runtime_lua_chunk_iterator_has_call_operator() {
        let Some(json) = runtime_json() else {
            return;
        };
        let root: RuntimeApiRoot = serde_json::from_str(&json).unwrap();

        let cls = root
            .classes
            .iter()
            .find(|c| c.basic_member.name == "LuaChunkIterator")
            .expect("LuaChunkIterator should exist");

        assert_eq!(cls.operators.len(), 1);
        assert!(matches!(cls.operators[0], Operator::Call(_)));
    }

    #[test]
    fn runtime_lua_bootstrap_raise_event_has_raises() {
        let Some(json) = runtime_json() else {
            return;
        };
        let root: RuntimeApiRoot = serde_json::from_str(&json).unwrap();

        let cls = root
            .classes
            .iter()
            .find(|c| c.basic_member.name == "LuaBootstrap")
            .expect("LuaBootstrap should exist");

        let method = cls
            .methods
            .iter()
            .find(|m| m.basic_member.name == "raise_biter_base_built")
            .expect("method should exist");

        let raises = method.raises.as_ref().expect("raises should be present");
        assert!(!raises.is_empty());
        assert_eq!(raises[0].name, "on_biter_base_built");
    }

    #[test]
    fn runtime_lua_object_concept_is_builtin() {
        let Some(json) = runtime_json() else {
            return;
        };
        let root: RuntimeApiRoot = serde_json::from_str(&json).unwrap();

        let concept = root
            .concepts
            .iter()
            .find(|c| c.basic_member.name == "LuaObject")
            .expect("LuaObject concept should exist");

        let RuntimeType::Complex(inner) = &concept.ty else {
            panic!("LuaObject type should be Complex, not Simple")
        };
        assert!(matches!(**inner, RuntimeComplexType::Builtin));
    }

    #[test]
    fn runtime_on_built_entity_has_filter() {
        let Some(json) = runtime_json() else {
            return;
        };
        let root: RuntimeApiRoot = serde_json::from_str(&json).unwrap();

        let event = root
            .events
            .iter()
            .find(|e| e.basic_member.name == "on_built_entity")
            .expect("on_built_entity should exist");

        assert!(event.filter.is_some());
        assert!(!event.data.is_empty());
    }

    #[test]
    fn runtime_defines_control_behavior_is_recursive() {
        let Some(json) = runtime_json() else {
            return;
        };
        let root: RuntimeApiRoot = serde_json::from_str(&json).unwrap();

        let define = root
            .defines
            .iter()
            .find(|d| d.basic_member.name == "control_behavior")
            .expect("control_behavior define should exist");

        let subkeys = define.subkeys.as_ref().expect("should have subkeys");
        assert!(!subkeys.is_empty());
        assert!(subkeys.iter().any(|s| s.subkeys.is_some()));
    }

    #[test]
    fn runtime_global_objects_include_game() {
        let Some(json) = runtime_json() else {
            return;
        };
        let root: RuntimeApiRoot = serde_json::from_str(&json).unwrap();

        assert!(root.global_objects.iter().any(|g| g.name == "game"));
        assert!(root.global_objects.iter().any(|g| g.name == "script"));
    }

    #[test]
    fn prototype_accumulator_prototype_has_typename() {
        let Some(json) = prototype_json() else {
            return;
        };
        let root: PrototypeApiRoot = serde_json::from_str(&json).unwrap();

        let p = root
            .prototypes
            .iter()
            .find(|p| p.basic_member.name == "AccumulatorPrototype")
            .expect("AccumulatorPrototype should exist");

        assert_eq!(p.typename.as_deref(), Some("accumulator"));
        assert!(!p.r#abstract);
        assert!(!p.deprecated);
    }

    #[test]
    fn prototype_player_port_is_deprecated() {
        let Some(json) = prototype_json() else {
            return;
        };
        let root: PrototypeApiRoot = serde_json::from_str(&json).unwrap();

        let p = root
            .prototypes
            .iter()
            .find(|p| p.basic_member.name == "PlayerPortPrototype")
            .expect("PlayerPortPrototype should exist");

        assert!(p.deprecated);
    }

    #[test]
    fn prototype_gui_style_has_custom_properties() {
        let Some(json) = prototype_json() else {
            return;
        };
        let root: PrototypeApiRoot = serde_json::from_str(&json).unwrap();

        let p = root
            .prototypes
            .iter()
            .find(|p| p.basic_member.name == "GuiStyle")
            .expect("GuiStyle should exist");

        assert!(p.custom_properties.is_some());
    }

    #[test]
    fn prototype_activate_equipment_is_struct_with_properties() {
        let Some(json) = prototype_json() else {
            return;
        };
        let root: PrototypeApiRoot = serde_json::from_str(&json).unwrap();

        let t = root
            .types
            .iter()
            .find(|t| t.basic_member.name == "ActivateEquipmentCapsuleAction")
            .expect("type should exist");

        let PrototypeType::Complex(inner) = &t.ty else {
            panic!()
        };
        assert!(matches!(**inner, PrototypeComplexType::Struct));
        assert!(t.properties.as_ref().map_or(false, |p| !p.is_empty()));
    }

    #[test]
    fn prototype_ammo_category_has_instance_limit() {
        let Some(json) = prototype_json() else {
            return;
        };
        let root: PrototypeApiRoot = serde_json::from_str(&json).unwrap();

        let p = root
            .prototypes
            .iter()
            .find(|p| p.basic_member.name == "AmmoCategory")
            .expect("AmmoCategory should exist");

        assert_eq!(p.instance_limit, Some(255));
    }

    #[test]
    fn runtime_and_prototype_define_counts_match() {
        let (Some(runtime_json), Some(proto_json)) = (runtime_json(), prototype_json()) else {
            return;
        };

        let runtime: RuntimeApiRoot = serde_json::from_str(&runtime_json).unwrap();
        let proto: PrototypeApiRoot = serde_json::from_str(&proto_json).unwrap();

        assert_eq!(
            runtime.defines.len(),
            proto.defines.len(),
            "runtime and prototype define counts should match"
        );
    }

    #[test]
    fn runtime_all_complex_types_deserialize() {
        let Some(json) = runtime_json() else {
            return;
        };
        let _: RuntimeApiRoot = serde_json::from_str(&json)
            .expect("all complex_type variants in runtime-api.json must deserialize");
    }

    #[test]
    fn prototype_all_complex_types_deserialize() {
        let Some(json) = prototype_json() else {
            return;
        };
        let _: PrototypeApiRoot = serde_json::from_str(&json)
            .expect("all complex_type variants in prototype-api.json must deserialize");
    }
}
