use assembler_schema::prelude::*;
use quote::{format_ident, quote};

pub fn map_type(ty: &RuntimeType) -> proc_macro2::TokenStream {
    match ty {
        RuntimeType::Simple(s) => map_simple(s),
        RuntimeType::Complex(c) => map_complex(c),
    }
}

fn map_simple(name: &str) -> proc_macro2::TokenStream {
    match name {
        "string" => quote! { String },
        "boolean" => quote! { bool },
        "uint" => quote! { u32 },
        "uint8" => quote! { u8 },
        "uint16" => quote! { u16 },
        "uint32" => quote! { u32 },
        "uint64" => quote! { u64 },
        "int" => quote! { i32 },
        "int8" => quote! { i8 },
        "double" => quote! { f64 },
        "float" => quote! { f32 },
        "nil" => quote! { () },
        "Any" => quote! { LuaAnyValue },
        other => {
            let i = format_ident!("{}", sanitize_ident(other));
            quote! { #i }
        }
    }
}

fn map_complex(ty: &RuntimeComplexType) -> proc_macro2::TokenStream {
    match ty {
        RuntimeComplexType::Type { value, .. } => map_type(value),
        RuntimeComplexType::Array { value } => {
            let inner = map_type(value);
            quote! { Vec<#inner> }
        }
        RuntimeComplexType::Dictionary { key, value } => {
            let k = map_type(key);
            let v = map_type(value);
            quote! { HashMap<#k, #v> }
        }

        RuntimeComplexType::Tuple { values } => {
            let inner: Vec<_> = values.iter().map(map_type).collect();
            match inner.len() {
                0 => quote! { () },
                1 => inner.into_iter().next().unwrap(),
                _ => quote! { (#(#inner),*) },
            }
        }
        RuntimeComplexType::Union { options, .. } => map_union(options),
        RuntimeComplexType::Literal { value, .. } => map_literal_value(value),
        RuntimeComplexType::Table { .. } => {
            // inline table types are opaque, named structs are emitted later
            quote! { LuaTable }
        }
        RuntimeComplexType::Function { .. } => quote! { LuaFunction },
        RuntimeComplexType::LuaLazyLoadedValue { value } => {
            let inner = map_type(value);
            quote! { LuaLazyLoadedValue<#inner> }
        }
        RuntimeComplexType::LuaStruct { .. } => quote! { LuaStruct },
        RuntimeComplexType::Builtin => quote! { LuaBuiltin },
    }
}

// strip nil, then count how many non-nil arms remain
// 0 arms -> ()
// 1 arm -> Option<T> if original was nil, else T
// 2+ arms -> generate an enum
fn map_union(options: &[RuntimeType]) -> proc_macro2::TokenStream {
    let (nils, rest): (Vec<_>, Vec<_>) = options
        .iter()
        .partition(|o| matches!(o, RuntimeType::Simple(s) if s == "nil"));

    let nullable = !nils.is_empty();

    let non_nil: Vec<proc_macro2::TokenStream> = rest.iter().map(|t| map_type(t)).collect();

    let inner = match non_nil.len() {
        0 => return quote! { () },
        1 => non_nil.into_iter().next().unwrap(),
        _ => {
            let unique = dedup_token_streams(non_nil);
            if unique.len() == 1 {
                unique.into_iter().next().unwrap()
            } else {
                let arity = format_ident!("Union{}", unique.len());
                quote! { #arity<#(#unique),*> }
            }
        }
    };

    if nullable {
        quote! { Option<#inner> }
    } else {
        inner
    }
}

pub fn map_literal_value(value: &RuntimeLiteralValue) -> proc_macro2::TokenStream {
    match value {
        RuntimeLiteralValue::String(_) => quote! { &'static str },
        RuntimeLiteralValue::Number(n) => {
            if n.fract() == 0.0 && n.abs() < i64::MAX as f64 {
                quote! { i64 }
            } else {
                quote! { f64 }
            }
        }
        RuntimeLiteralValue::Bool(_) => quote! { bool },
    }
}

fn sanitize_ident(s: &str) -> String {
    s.replace('.', "__")
}

fn dedup_token_streams(streams: Vec<proc_macro2::TokenStream>) -> Vec<proc_macro2::TokenStream> {
    let mut seen = std::collections::HashSet::new();
    streams
        .into_iter()
        .filter(|ts| seen.insert(ts.to_string()))
        .collect()
}
