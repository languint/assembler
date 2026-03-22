mod arrays;
mod dictionary;
mod literal;
mod opaque;
mod simple;
mod tuple;
mod type_wrapper;
mod union;

use assembler_codegen::emit::types::map_type;
use assembler_schema::prelude::*;

fn ts(ty: &RuntimeType) -> String {
    map_type(ty).to_string()
}

fn simple(s: &str) -> RuntimeType {
    RuntimeType::Simple(s.to_string())
}

fn complex(c: RuntimeComplexType) -> RuntimeType {
    RuntimeType::Complex(Box::new(c))
}
