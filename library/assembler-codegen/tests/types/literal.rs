#[cfg(test)]
mod literal {
    use crate::types::{complex, ts};
    use assembler_schema::prelude::*;

    #[test]
    fn literal_string() {
        let ty = complex(RuntimeComplexType::Literal {
            value: RuntimeLiteralValue::String("resource".into()),
            description: None,
        });
        assert_eq!(ts(&ty), "& 'static str");
    }

    #[test]
    fn literal_integer() {
        let ty = complex(RuntimeComplexType::Literal {
            value: RuntimeLiteralValue::Number(0.0),
            description: None,
        });
        assert_eq!(ts(&ty), "i64");
    }

    #[test]
    fn literal_float() {
        let ty = complex(RuntimeComplexType::Literal {
            value: RuntimeLiteralValue::Number(1.5),
            description: None,
        });
        assert_eq!(ts(&ty), "f64");
    }

    #[test]
    fn literal_bool() {
        let ty = complex(RuntimeComplexType::Literal {
            value: RuntimeLiteralValue::Bool(true),
            description: None,
        });
        assert_eq!(ts(&ty), "bool");
    }
}
