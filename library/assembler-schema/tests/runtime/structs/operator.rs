#[cfg(test)]
mod operator {
    use crate::utils::de;
    use assembler_schema::prelude::*;

    #[test]
    fn call_operator() {
        let o: Operator = de(r#"{
            "name": "call",
            "order": 0,
            "description": "Gets the next chunk position if the iterator is not yet done.",
            "parameters": [],
            "format": {"takes_table": false},
            "return_values": [{
                "name": "",
                "order": 0,
                "description": "",
                "type": "ChunkPositionAndArea",
                "optional": true
            }]
        }"#);
        let Operator::Call(m) = o else {
            panic!("expected Call")
        };
        assert_eq!(m.return_values.len(), 1);
        assert!(m.return_values[0].optional);
    }

    #[test]
    fn index_operator() {
        let o: Operator = de(r#"{
            "name": "index",
            "order": 0,
            "description": "The indexing operator.",
            "examples": ["```\ngame.player.get_main_inventory()[1]\n```"],
            "read_type": "LuaItemStack",
            "optional": false
        }"#);
        let Operator::Index(a) = o else {
            panic!("expected Index")
        };
        assert_eq!(a.basic_member.name, "index");
        assert!(!a.optional);
    }

    #[test]
    fn index_operator_optional() {
        let o: Operator = de(r#"{
            "name": "index",
            "order": 0,
            "description": "Gets children by name.",
            "read_type": "LuaGuiElement",
            "optional": true
        }"#);
        let Operator::Index(a) = o else { panic!() };
        assert!(a.optional);
    }

    #[test]
    fn length_operator() {
        let o: Operator = de(r#"{
            "name": "length",
            "order": 0,
            "description": "Returns the number of elements.",
            "read_type": "uint",
            "optional": false
        }"#);
        assert!(matches!(o, Operator::Length(_)));
    }
}
