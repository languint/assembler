#[cfg(test)]
mod opaque {
    use crate::types::{complex, simple, ts};
    use assembler_schema::prelude::*;

    #[test]
    fn function_is_lua_function() {
        let ty = complex(RuntimeComplexType::Function { parameters: vec![] });
        assert_eq!(ts(&ty), "LuaFunction");
    }

    #[test]
    fn table_is_lua_table() {
        let ty = complex(RuntimeComplexType::Table {
            parameters: vec![],
            variant_parameter_groups: None,
            variant_parameter_description: None,
        });
        assert_eq!(ts(&ty), "LuaTable");
    }

    #[test]
    fn lua_struct_is_opaque() {
        let ty = complex(RuntimeComplexType::LuaStruct { attributes: vec![] });
        assert_eq!(ts(&ty), "LuaStruct");
    }

    #[test]
    fn builtin_is_lua_builtin() {
        let ty = complex(RuntimeComplexType::Builtin);
        assert_eq!(ts(&ty), "LuaBuiltin");
    }

    #[test]
    fn lazy_loaded_value_wraps_inner() {
        let ty = complex(RuntimeComplexType::LuaLazyLoadedValue {
            value: complex(RuntimeComplexType::Dictionary {
                key: simple("uint32"),
                value: simple("LuaEntity"),
            }),
        });
        assert_eq!(
            ts(&ty),
            "LuaLazyLoadedValue < HashMap < u32 , LuaEntity > >"
        );
    }
}
