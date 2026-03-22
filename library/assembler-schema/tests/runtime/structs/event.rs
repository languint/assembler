#[cfg(test)]
mod event {
    use crate::utils::de;
    use assembler_schema::schema::prelude::*;

    #[test]
    fn event_without_filter() {
        let e: Event = de(r#"{
            "name": "on_tick",
            "order": 0,
            "description": "Called every tick.",
            "data": [{
                "name": "tick",
                "order": 0,
                "description": "The current game tick.",
                "type": "uint32",
                "optional": false
            }]
        }"#);
        assert_eq!(e.basic_member.name, "on_tick");
        assert_eq!(e.data.len(), 1);
        assert!(e.filter.is_none());
    }

    #[test]
    fn event_with_filter() {
        let e: Event = de(r#"{
            "name": "on_built_entity",
            "order": 7,
            "description": "Called when player builds something.",
            "data": [{
                "name": "entity",
                "order": 0,
                "description": "",
                "type": "LuaEntity",
                "optional": false
            }],
            "filter": "LuaEntityBuiltEventFilter"
        }"#);
        assert_eq!(e.filter.as_deref(), Some("LuaEntityBuiltEventFilter"));
    }

    #[test]
    fn event_with_empty_data() {
        let e: Event = de(r#"{"name":"on_init","order":0,"description":"","data":[]}"#);
        assert!(e.data.is_empty());
    }
}
