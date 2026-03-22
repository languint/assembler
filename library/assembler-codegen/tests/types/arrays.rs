#[cfg(test)]
mod arrays {
    use crate::types::{complex, simple, ts};
    use assembler_schema::prelude::*;

    #[test]
    fn array_of_simple() {
        let ty = complex(RuntimeComplexType::Array {
            value: simple("LuaEntity"),
        });
        assert_eq!(ts(&ty), "Vec < LuaEntity >");
    }

    #[test]
    fn array_of_array() {
        let inner = complex(RuntimeComplexType::Array {
            value: simple("string"),
        });
        let ty = complex(RuntimeComplexType::Array { value: inner });
        assert_eq!(ts(&ty), "Vec < Vec < String > >");
    }
}
