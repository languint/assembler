#[cfg(test)]
mod dictionary {
    use crate::types::{complex, simple, ts};
    use assembler_schema::prelude::*;

    #[test]
    fn dictionary() {
        let ty = complex(RuntimeComplexType::Dictionary {
            key: simple("string"),
            value: simple("LuaEntity"),
        });
        assert_eq!(ts(&ty), "HashMap < String , LuaEntity >");
    }
}
