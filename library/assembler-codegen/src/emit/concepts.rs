use assembler_schema::prelude::*;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::emit::sanitize_ident;
use crate::emit::types::{map_simple, map_type};

pub fn emit_concept(concept: &RuntimeConcept) -> TokenStream {
    let name = format_ident!("{}", concept.basic_member.name);
    let doc = &concept.basic_member.description;
    if concept.basic_member.name == "LocalisedString" {
        return quote! {
            #[doc = #doc]
            #[derive(Debug, Clone)]
            pub struct LocalisedString(pub Vec<LuaAnyValue>);
        };
    }

    match &concept.ty {
        RuntimeType::Complex(c) if matches!(c.as_ref(), RuntimeComplexType::Builtin) => {
            let mapped = match concept.basic_member.name.as_str() {
                "LuaObject" => quote! { LuaAnyValue },
                other => map_simple(other),
            };
            quote! {
                #[doc = #doc]
                pub type #name = #mapped;
            }
        }
        RuntimeType::Simple(s) => {
            let ty = map_simple(s);
            quote! {
                #[doc = #doc]
                pub type #name = #ty;
            }
        }
        RuntimeType::Complex(c) => match c.as_ref() {
            RuntimeComplexType::Union { options, .. } if is_table_tuple_union(options) => {
                emit_table_tuple_struct(&name, doc, options)
            }
            RuntimeComplexType::Union { options, .. } if all_string_literals(options) => {
                emit_string_enum(&name, doc, options)
            }
            RuntimeComplexType::Union { options, .. } => emit_named_union_enum(&name, doc, options),
            RuntimeComplexType::LuaStruct { attributes } => emit_lua_struct(&name, doc, attributes),
            RuntimeComplexType::Table {
                parameters,
                variant_parameter_groups,
                ..
            } => emit_table_struct(&name, doc, parameters, variant_parameter_groups.as_deref()),
            _ => {
                let ty = map_type(&RuntimeType::Complex(c.clone()));
                quote! {
                    #[doc = #doc]
                    pub type #name = #ty;
                }
            }
        },
    }
}

fn is_table_tuple_union(options: &[RuntimeType]) -> bool {
    let has_tuple = options.iter().any(|o| {
        matches!(o, RuntimeType::Complex(c)
            if matches!(c.as_ref(), RuntimeComplexType::Tuple { .. }))
    });

    has_tuple
        && options.iter().any(|o| {
            if let RuntimeType::Complex(c) = o
                && let RuntimeComplexType::Table { parameters, .. } = c.as_ref()
            {
                return !parameters.is_empty();
            }
            false
        })
}

fn emit_table_tuple_struct(name: &syn::Ident, doc: &str, options: &[RuntimeType]) -> TokenStream {
    let parameters = options
        .iter()
        .find_map(|o| {
            if let RuntimeType::Complex(c) = o {
                if let RuntimeComplexType::Table { parameters, .. } = c.as_ref() {
                    if !parameters.is_empty() {
                        return Some(parameters.as_slice());
                    }
                }
            }
            None
        })
        .expect("is_table_tuple_union passed but no table found");

    let tuple_values: Option<&Vec<RuntimeType>> = options.iter().find_map(|o| {
        if let RuntimeType::Complex(c) = o {
            if let RuntimeComplexType::Tuple { values } = c.as_ref() {
                return Some(values);
            }
        }
        None
    });

    let fields = parameters.iter().map(|p| {
        let fname = format_ident!("{}", sanitize_ident(&p.name));
        let fdoc = &p.description;
        let fty = map_type(&p.ty);
        if p.optional {
            quote! { #[doc = #fdoc] pub #fname: Option<#fty>, }
        } else {
            quote! { #[doc = #fdoc] pub #fname: #fty, }
        }
    });

    let tuple_from = tuple_values.map(|values| {
        let tuple_types: Vec<_> = values.iter().map(map_type).collect();
        let tuple_ty = quote! { (#(#tuple_types),*) };

        let required: Vec<&Parameter> = parameters.iter().filter(|p| !p.optional).collect();

        let field_assignments: Vec<TokenStream> = if required.is_empty() {
            parameters
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let fname = format_ident!("{}", sanitize_ident(&p.name));
                    let idx = syn::Index::from(i);
                    quote! { #fname: Some(val.#idx), }
                })
                .collect()
        } else if required.len() == parameters.len() {
            parameters
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let fname = format_ident!("{}", sanitize_ident(&p.name));
                    let idx = syn::Index::from(i);
                    quote! { #fname: val.#idx, }
                })
                .collect()
        } else {
            assert_eq!(
                values.len(),
                required.len(),
                "tuple arity {} != required field count {}",
                values.len(),
                required.len()
            );
            let mut req_idx = 0usize;
            parameters
                .iter()
                .map(|p| {
                    let fname = format_ident!("{}", sanitize_ident(&p.name));
                    if p.optional {
                        quote! { #fname: None, }
                    } else {
                        let idx = syn::Index::from(req_idx);
                        req_idx += 1;
                        quote! { #fname: val.#idx, }
                    }
                })
                .collect()
        };

        quote! {
            impl From<#tuple_ty> for #name {
                fn from(val: #tuple_ty) -> Self {
                    Self { #(#field_assignments)* }
                }
            }
        }
    });

    let field_from_impls = if parameters.len() == 2 {
        let f0 = format_ident!("{}", sanitize_ident(&parameters[0].name));
        let f1 = format_ident!("{}", sanitize_ident(&parameters[1].name));
        let t0 = map_type(&parameters[0].ty);
        let t1 = map_type(&parameters[1].ty);
        quote! {
            impl #name {
                pub fn new(#f0: #t0, #f1: #t1) -> Self {
                    Self { #f0, #f1 }
                }
            }
        }
    } else {
        quote! {}
    };

    quote! {
        #[doc = #doc]
        #[derive(Debug, Clone, PartialEq)]
        pub struct #name {
            #(#fields)*
        }

        #tuple_from
        #field_from_impls
    }
}

fn emit_named_union_enum(name: &syn::Ident, doc: &str, options: &[RuntimeType]) -> TokenStream {
    let (nils, rest): (Vec<_>, Vec<_>) = options
        .iter()
        .partition(|o| matches!(o, RuntimeType::Simple(s) if s == "nil"));

    let nullable = !nils.is_empty();
    let non_nil: Vec<_> = rest.iter().collect();

    if non_nil.is_empty() {
        return quote! { pub type #name = (); };
    }

    let mut seen_variants: std::collections::HashSet<String> = Default::default();
    let variants_and_types: Vec<(syn::Ident, TokenStream)> = non_nil
        .iter()
        .filter_map(|opt| {
            let variant_name = type_to_variant_name(opt);
            let rust_ty = map_type(opt);

            if !seen_variants.insert(variant_name.clone()) {
                return None;
            }

            let vident = format_ident!("{}", variant_name);
            Some((vident, rust_ty))
        })
        .collect();

    let variants = variants_and_types.iter().map(|(vname, ty)| {
        quote! { #vname(#ty), }
    });

    let from_impls = {
        let mut seen_types: std::collections::HashSet<String> = Default::default();

        variants_and_types
            .iter()
            .filter_map(|(vname, ty)| {
                let ty_str = ty.to_string();

                if !seen_types.insert(ty_str) {
                    return None;
                }

                Some(if nullable {
                    quote! {
                        impl From<#ty> for Option<#name> {
                            fn from(val: #ty) -> Self {
                                Some(#name::#vname(val))
                            }
                        }
                    }
                } else {
                    quote! {
                        impl From<#ty> for #name {
                            fn from(val: #ty) -> Self {
                                #name::#vname(val)
                            }
                        }
                    }
                })
            })
            .collect::<Vec<_>>()
    };

    quote! {
        #[doc = #doc]
        #[derive(Debug, Clone)]
        pub enum #name {
            #(#variants)*
        }
        #(#from_impls)*
    }
}

pub fn type_to_variant_name(ty: &RuntimeType) -> String {
    match ty {
        RuntimeType::Simple(s) => simple_to_variant_name(s),
        RuntimeType::Complex(c) => complex_to_variant_name(c),
    }
}

fn simple_to_variant_name(name: &str) -> String {
    match name {
        "string" => "String".into(),
        "boolean" => "Bool".into(),
        "uint" => "Uint".into(),
        "uint8" => "Uint8".into(),
        "uint16" => "Uint16".into(),
        "uint32" => "Uint32".into(),
        "uint64" => "Uint64".into(),
        "int" => "Int".into(),
        "int8" => "Int8".into(),
        "int16" => "Int16".into(),
        "int32" => "Int32".into(),
        "double" => "Double".into(),
        "float" => "Float".into(),
        "nil" => "Nil".into(),
        "Any" => "Any".into(),
        "table" => "Table".into(),
        other => {
            if other.starts_with("Lua") && other.len() > 3 {
                other[3..].to_string()
            } else {
                other.replace('.', "__")
            }
        }
    }
}

fn complex_to_variant_name(c: &RuntimeComplexType) -> String {
    match c {
        RuntimeComplexType::Array { value } => {
            format!("ArrayOf{}", type_to_variant_name(value))
        }
        RuntimeComplexType::Dictionary { key, value } => {
            format!(
                "{}To{}Map",
                type_to_variant_name(key),
                type_to_variant_name(value)
            )
        }
        RuntimeComplexType::Tuple { values } => {
            if values.len() == 2 {
                format!(
                    "{}{}Pair",
                    type_to_variant_name(&values[0]),
                    type_to_variant_name(&values[1])
                )
            } else {
                format!("Tuple{}", values.len())
            }
        }
        RuntimeComplexType::Table { .. } => "Table".into(),
        RuntimeComplexType::Literal { value, .. } => match value {
            RuntimeLiteralValue::String(s) => sanitize_enum_variant(s),
            RuntimeLiteralValue::Number(_) => "Number".into(),
            RuntimeLiteralValue::Bool(b) => {
                if *b {
                    "True".into()
                } else {
                    "False".into()
                }
            }
        },
        RuntimeComplexType::Type { value, .. } => type_to_variant_name(value),
        RuntimeComplexType::Union { .. } => "Union".into(),
        RuntimeComplexType::Function { .. } => "Function".into(),
        RuntimeComplexType::LuaLazyLoadedValue { value } => {
            format!("Lazy{}", type_to_variant_name(value))
        }
        RuntimeComplexType::LuaStruct { .. } => "Struct".into(),
        RuntimeComplexType::Builtin => "Builtin".into(),
    }
}

fn emit_string_enum(name: &syn::Ident, doc: &str, options: &[RuntimeType]) -> TokenStream {
    let mut seen = std::collections::HashSet::new();
    let variants = options.iter().filter_map(|opt| {
        let RuntimeType::Complex(c) = opt else {
            return None;
        };
        let RuntimeComplexType::Literal {
            value: RuntimeLiteralValue::String(s),
            description,
        } = c.as_ref()
        else {
            return None;
        };

        let variant_name = sanitize_enum_variant(s);
        if !seen.insert(variant_name.clone()) {
            return None;
        }

        let vname = format_ident!("{}", variant_name);
        let vdoc = description.as_deref().unwrap_or("");
        let original_note = if variant_name != *s {
            let note = format!("Lua value: `\"{s}\"`");
            quote! { #[doc = #note] }
        } else {
            quote! {}
        };

        Some(quote! {
            #[doc = #vdoc]
            #original_note
            #vname,
        })
    });

    quote! {
        #[doc = #doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum #name {
            #(#variants)*
        }
    }
}

fn emit_table_struct(
    name: &syn::Ident,
    doc: &str,
    parameters: &[Parameter],
    variant_parameter_groups: Option<&[ParameterGroup]>,
) -> TokenStream {
    let struct_name = name.to_string();

    let fields = parameters.iter().map(|p| {
        let fname = format_ident!("{}", sanitize_ident(&p.name));
        let fdoc = &p.description;
        let mut fty = map_type(&p.ty);

        if matches!(&p.ty, RuntimeType::Simple(s) if s == &struct_name) {
            fty = quote! { Box<#fty> };
        }

        if p.optional {
            quote! { #[doc = #fdoc] pub #fname: Option<#fty>, }
        } else {
            quote! { #[doc = #fdoc] pub #fname: #fty, }
        }
    });

    let extra = if variant_parameter_groups.is_some_and(|g| !g.is_empty()) {
        let groups: Vec<String> = variant_parameter_groups
            .unwrap()
            .iter()
            .map(|g| g.name.clone())
            .collect();
        let note = format!(
            "Variant parameter groups: {}. Construct the appropriate variant fields directly.",
            groups.join(", ")
        );
        quote! {
            #[doc = #note]
            pub extra: Option<LuaTable>,
        }
    } else {
        quote! {}
    };

    quote! {
        #[doc = #doc]
        #[derive(Debug, Clone)]
        pub struct #name {
            #(#fields)*
            #extra
        }
    }
}

fn emit_lua_struct(name: &syn::Ident, doc: &str, attributes: &[Attribute]) -> TokenStream {
    let fields = attributes.iter().filter_map(|a| {
        let ty = a.read_type.as_ref().or(a.write_type.as_ref())?;

        let fname = format_ident!("{}", sanitize_ident(&a.basic_member.name));
        let fdoc = &a.basic_member.description;
        let fty = map_type(ty);

        Some(if a.optional {
            quote! { #[doc = #fdoc] pub #fname: Option<#fty>, }
        } else {
            quote! { #[doc = #fdoc] pub #fname: #fty, }
        })
    });

    quote! {
        #[doc = #doc]
        #[derive(Debug, Clone)]
        pub struct #name {
            #(#fields)*
        }
    }
}

fn all_string_literals(options: &[RuntimeType]) -> bool {
    !options.is_empty()
        && options.iter().all(|o| {
            matches!(
                o,
                RuntimeType::Complex(c)
                if matches!(
                    c.as_ref(),
                    RuntimeComplexType::Literal { value: RuntimeLiteralValue::String(_), .. }
                )
            )
        })
}

fn sanitize_enum_variant(s: &str) -> String {
    if s.is_empty() {
        return "None_".into();
    }

    let named = match s {
        "*" => "Multiply",
        "/" => "Divide",
        "+" => "Add",
        "-" => "Subtract",
        "%" => "Modulo",
        "^" => "Power",
        "<<" => "LeftShift",
        ">>" => "RightShift",
        "=" => "Equal",
        "!=" => "NotEqual",
        "<" => "LessThan",
        ">" => "GreaterThan",
        "<=" => "LessThanOrEqual",
        ">=" => "GreaterThanOrEqual",
        "≠" => "NotEqual",
        "≤" => "LessThanOrEqual",
        "≥" => "GreaterThanOrEqual",
        "AND" => "And",
        "OR" => "Or",
        "XOR" => "Xor",
        _ => "",
    };
    if !named.is_empty() {
        return named.into();
    }

    let needs_escape = crate::emit::is_keyword(s);
    let pascal: String = s
        .split(|c: char| c == '-' || c == '_' || c == ' ')
        .filter(|p| !p.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect();

    if !pascal.chars().all(|c| c.is_alphanumeric() || c == '_') || pascal.is_empty() {
        panic!("sanitize_enum_variant: unmapped symbol {s:?} → {pascal:?}");
    }

    if needs_escape {
        format!("{pascal}_")
    } else {
        pascal
    }
}
