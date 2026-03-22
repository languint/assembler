#[cfg(test)]
mod union {
    use crate::types::{complex, simple, ts};
    use assembler_schema::prelude::*;

    #[test]
    fn union_single_non_nil() {
        let ty = complex(RuntimeComplexType::Union {
            options: vec![simple("string")],
            full_format: false,
        });
        assert_eq!(ts(&ty), "String");
    }

    #[test]
    fn union_nullable_single() {
        let ty = complex(RuntimeComplexType::Union {
            options: vec![simple("string"), simple("nil")],
            full_format: false,
        });
        assert_eq!(ts(&ty), "Option < String >");
    }

    #[test]
    fn union_all_nil_is_unit() {
        let ty = complex(RuntimeComplexType::Union {
            options: vec![simple("nil")],
            full_format: false,
        });
        assert_eq!(ts(&ty), "()");
    }

    #[test]
    fn union_two_types() {
        let ty = complex(RuntimeComplexType::Union {
            options: vec![simple("string"), simple("uint32")],
            full_format: false,
        });
        assert_eq!(ts(&ty), "Union2 < String , u32 >");
    }

    #[test]
    fn union_two_types_nullable() {
        let ty = complex(RuntimeComplexType::Union {
            options: vec![simple("string"), simple("uint32"), simple("nil")],
            full_format: false,
        });
        assert_eq!(ts(&ty), "Option < Union2 < String , u32 > >");
    }

    #[test]
    fn union_deduplicates() {
        let ty = complex(RuntimeComplexType::Union {
            options: vec![simple("string"), simple("string")],
            full_format: false,
        });
        assert_eq!(ts(&ty), "String");
    }
}
