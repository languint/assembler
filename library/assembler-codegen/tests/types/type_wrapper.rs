#[cfg(test)]
mod type_wrapper {
    use crate::types::{complex, simple, ts};
    use assembler_schema::prelude::*;

    #[test]
    fn type_wrapper_unwraps() {
        let ty = complex(RuntimeComplexType::Type {
            value: simple("LuaEntity"),
            description: "An entity.".into(),
        });
        assert_eq!(ts(&ty), "LuaEntity");
    }
}
