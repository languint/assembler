#[cfg(test)]
mod simple {
    use crate::types::{simple, ts};

    #[test]
    fn simple_primitives() {
        assert_eq!(ts(&simple("string")), "String");
        assert_eq!(ts(&simple("boolean")), "bool");
        assert_eq!(ts(&simple("uint32")), "u32");
        assert_eq!(ts(&simple("double")), "f64");
        assert_eq!(ts(&simple("nil")), "()");
        assert_eq!(ts(&simple("Any")), "LuaAnyValue");
    }

    #[test]
    fn simple_named_class() {
        assert_eq!(ts(&simple("LuaEntity")), "LuaEntity");
    }

    #[test]
    fn simple_dotted_ident_is_sanitized() {
        assert_eq!(ts(&simple("defines.inventory")), "defines__inventory");
    }
}
