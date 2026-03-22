#[cfg(test)]
mod tuple {
    use crate::types::{complex, simple, ts};
    use assembler_schema::prelude::*;

    #[test]
    fn tuple_single_unwraps() {
        let ty = complex(RuntimeComplexType::Tuple {
            values: vec![simple("uint32")],
        });
        assert_eq!(ts(&ty), "u32");
    }

    #[test]
    fn tuple_empty_is_unit() {
        let ty = complex(RuntimeComplexType::Tuple { values: vec![] });
        assert_eq!(ts(&ty), "()");
    }

    #[test]
    fn tuple_multiple() {
        let ty = complex(RuntimeComplexType::Tuple {
            values: vec![simple("uint32"), simple("uint32")],
        });
        assert_eq!(ts(&ty), "(u32 , u32)");
    }
}
