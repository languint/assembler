use assembler_schema::prelude::*;
use quote::{format_ident, quote};

use crate::emit::sanitize_ident;
use crate::emit::types::{map_simple, map_type};

pub fn emit_concept(concept: &RuntimeConcept) -> proc_macro2::TokenStream {
    if concept.basic_member.name == "LocalisedString" {
        let doc = &concept.basic_member.description;
        return quote! {
            #[doc = #doc]
            #[derive(Debug, Clone)]
            pub struct LocalisedString(Vec<LuaAnyValue>);
        };
    }

    let name = format_ident!("{}", concept.basic_member.name);
    let doc = &concept.basic_member.description;

    let body = match &concept.ty {
        // named type or primitive -> type alias
        RuntimeType::Simple(s) => emit_simple_alias(&name, doc, s),
        RuntimeType::Complex(c) if matches!(c.as_ref(), RuntimeComplexType::Builtin) => {
            let doc = &concept.basic_member.description;

            let mapped = match concept.basic_member.name.as_str() {
                "LuaObject" => quote! { LuaAnyValue },
                other => map_simple(other),
            };
            return quote! {
                #[doc = #doc]
                pub type #name = #mapped;
            };
        }
        RuntimeType::Complex(c) => match c.as_ref() {
            // table -> struct
            RuntimeComplexType::Table {
                parameters,
                variant_parameter_groups,
                ..
            } => emit_table_struct(&name, doc, parameters, variant_parameter_groups.as_deref()),

            // union w/ all string literals -> rust enum
            // union -> type alias via `map_union`
            RuntimeComplexType::Union { options, .. } => {
                if all_string_literals(options) {
                    emit_string_enum(&name, doc, options)
                } else {
                    emit_type_alias(&name, doc, &RuntimeType::Complex(c.clone()))
                }
            }

            // LuaStruct -> struct whose fields come from the attributes
            RuntimeComplexType::LuaStruct { attributes } => emit_lua_struct(&name, doc, attributes),

            // type alias catchall
            _ => emit_type_alias(&name, doc, &RuntimeType::Complex(c.clone())),
        },
    };

    body
}

fn emit_simple_alias(name: &syn::Ident, doc: &str, s: &str) -> proc_macro2::TokenStream {
    let ty = crate::emit::types::map_simple(s);
    quote! {
        #[doc = #doc]
        pub type #name = #ty;
    }
}

fn emit_type_alias(name: &syn::Ident, doc: &str, ty: &RuntimeType) -> proc_macro2::TokenStream {
    let mapped = map_type(ty);
    quote! {
        #[doc = #doc]
        pub type #name = #mapped;
    }
}

fn emit_table_struct(
    name: &syn::Ident,
    doc: &str,
    parameters: &[Parameter],
    variant_parameter_groups: Option<&[ParameterGroup]>,
) -> proc_macro2::TokenStream {
    let struct_name = name.to_string();

    let fields = parameters.iter().map(|p| {
        let fname = format_ident!("{}", sanitize_ident(&p.name));
        let fdoc = &p.description;
        let mut fty = map_type(&p.ty);

        let is_recursive = matches!(&p.ty, RuntimeType::Simple(s) if *s == struct_name);
        if is_recursive {
            fty = quote! { Box<#fty> };
        }

        if p.optional {
            quote! { #[doc = #fdoc] pub #fname: Option<#fty>, }
        } else {
            quote! { #[doc = #fdoc] pub #fname: #fty, }
        }
    });

    let extra = if variant_parameter_groups.map_or(false, |g| !g.is_empty()) {
        let groups: Vec<String> = variant_parameter_groups
            .unwrap()
            .iter()
            .map(|g| g.name.clone())
            .collect();
        let note = format!(
            "variant parameter groups: {}. \
             construct the appropriate variant fields directly.",
            groups.join(", ")
        );
        quote! {
            #[doc = #note]
            #[allow(dead_code)]
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

fn emit_string_enum(
    name: &syn::Ident,
    doc: &str,
    options: &[RuntimeType],
) -> proc_macro2::TokenStream {
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
            return None; // skip duplicate
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

fn emit_lua_struct(
    name: &syn::Ident,
    doc: &str,
    attributes: &[Attribute],
) -> proc_macro2::TokenStream {
    let fields = attributes.iter().filter_map(|a| {
        let ty = a.read_type.as_ref().or(a.write_type.as_ref())?;

        let fname = format_ident!("{}", sanitize_ident(&a.basic_member.name));
        let fdoc = &a.basic_member.description;
        let fty = map_type(ty);

        Some(if a.optional {
            quote! {
                #[doc = #fdoc]
                pub #fname: Option<#fty>,
            }
        } else {
            quote! {
                #[doc = #fdoc]
                pub #fname: #fty,
            }
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
                    RuntimeComplexType::Literal {
                        value: RuntimeLiteralValue::String(_),
                        ..
                    }
                )
            )
        })
}

fn sanitize_enum_variant(s: &str) -> String {
    if s.is_empty() {
        return "None_".to_string();
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
        "AND" => "And",
        "OR" => "Or",
        "XOR" => "Xor",
        "=" => "Equal",
        "!=" => "NotEqual",
        "<" => "LessThan",
        ">" => "GreaterThan",
        "<=" => "LessThanOrEqual",
        ">=" => "GreaterThanOrEqual",
        "≠" => "NotEqual",
        "≤" => "LessThanOrEqual",
        "≥" => "GreaterThanOrEqual",
        _ => "",
    };

    if !named.is_empty() {
        return named.to_string();
    }

    let needs_escape = crate::emit::is_keyword(s);

    let pascal: String = s
        .split(|c: char| c == '-' || c == '_' || c == ' ')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect();

    if !pascal.chars().all(|c| c.is_alphanumeric() || c == '_') {
        panic!("sanitize_enum_variant: unmapped symbol {s:?} produced invalid ident {pascal:?}");
    }

    if needs_escape {
        format!("{pascal}_")
    } else {
        pascal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ts(concept: &RuntimeConcept) -> String {
        emit_concept(concept).to_string()
    }

    fn concept(name: &str, ty: RuntimeType) -> RuntimeConcept {
        RuntimeConcept {
            basic_member: BasicMember {
                name: name.to_string(),
                order: 0,
                description: String::new(),
                lists: None,
                examples: None,
                images: None,
            },
            ty,
        }
    }

    fn simple(s: &str) -> RuntimeType {
        RuntimeType::Simple(s.to_string())
    }

    fn complex(c: RuntimeComplexType) -> RuntimeType {
        RuntimeType::Complex(Box::new(c))
    }

    fn param(name: &str, ty: RuntimeType, optional: bool) -> Parameter {
        Parameter {
            name: name.to_string(),
            order: 0,
            description: String::new(),
            ty,
            optional,
        }
    }

    // --- Simple alias ---

    #[test]
    fn simple_primitive_alias() {
        let c = concept("MapTick", simple("uint64"));
        assert!(ts(&c).contains("pub type MapTick = u64"));
    }

    #[test]
    fn simple_named_alias() {
        let c = concept("CollisionLayerID", simple("string"));
        assert!(ts(&c).contains("pub type CollisionLayerID = String"));
    }

    // --- Type alias via map_type ---

    #[test]
    fn array_alias() {
        let c = concept(
            "SomeList",
            complex(RuntimeComplexType::Array {
                value: simple("string"),
            }),
        );
        assert!(ts(&c).contains("pub type SomeList = Vec < String >"));
    }

    #[test]
    fn dictionary_alias() {
        let c = concept(
            "EntityPrototypeFlags",
            complex(RuntimeComplexType::Dictionary {
                key: simple("string"),
                value: complex(RuntimeComplexType::Literal {
                    value: RuntimeLiteralValue::Bool(true),
                    description: None,
                }),
            }),
        );
        assert!(ts(&c).contains("pub type EntityPrototypeFlags"));
    }

    #[test]
    fn tuple_alias() {
        let c = concept(
            "BlueprintWire",
            complex(RuntimeComplexType::Tuple {
                values: vec![simple("uint32"), simple("uint32")],
            }),
        );
        assert!(ts(&c).contains("pub type BlueprintWire = (u32 , u32)"));
    }

    // --- Table struct ---

    #[test]
    fn table_struct_required_fields() {
        let c = concept(
            "AccumulatorControl",
            complex(RuntimeComplexType::Table {
                parameters: vec![
                    param("output_signal", simple("SignalID"), false),
                    param("read_charge", simple("boolean"), true),
                ],
                variant_parameter_groups: None,
                variant_parameter_description: None,
            }),
        );
        let out = ts(&c);
        assert!(out.contains("pub struct AccumulatorControl"));
        assert!(out.contains("pub output_signal : SignalID"));
        assert!(out.contains("pub read_charge : Option < bool >"));
    }

    #[test]
    fn table_struct_with_variant_groups_has_extra_field() {
        let c = concept(
            "FilterConcept",
            complex(RuntimeComplexType::Table {
                parameters: vec![param("filter", simple("string"), false)],
                variant_parameter_groups: Some(vec![ParameterGroup {
                    name: "type".to_string(),
                    order: 0,
                    description: String::new(),
                    parameters: vec![],
                }]),
                variant_parameter_description: None,
            }),
        );
        let out = ts(&c);
        assert!(out.contains("pub extra : Option < LuaTable >"));
    }

    // --- String enum ---

    #[test]
    fn all_string_literal_union_becomes_enum() {
        let c = concept(
            "Alignment",
            complex(RuntimeComplexType::Union {
                options: vec![
                    complex(RuntimeComplexType::Literal {
                        value: RuntimeLiteralValue::String("top-left".into()),
                        description: None,
                    }),
                    complex(RuntimeComplexType::Literal {
                        value: RuntimeLiteralValue::String("bottom-right".into()),
                        description: None,
                    }),
                ],
                full_format: false,
            }),
        );
        let out = ts(&c);
        assert!(out.contains("pub enum Alignment"));
        assert!(out.contains("TopLeft"));
        assert!(out.contains("BottomRight"));
    }

    #[test]
    fn mixed_union_becomes_type_alias() {
        let c = concept(
            "MapGenSize",
            complex(RuntimeComplexType::Union {
                options: vec![simple("float"), simple("string")],
                full_format: false,
            }),
        );
        let out = ts(&c);
        assert!(out.contains("pub type MapGenSize"));
        assert!(!out.contains("pub enum"));
    }

    // --- LuaStruct ---

    #[test]
    fn lua_struct_uses_read_type() {
        let c = concept(
            "DifficultySettings",
            complex(RuntimeComplexType::LuaStruct {
                attributes: vec![Attribute {
                    basic_member: BasicMember {
                        name: "technology_price_multiplier".to_string(),
                        order: 0,
                        description: String::new(),
                        lists: None,
                        examples: None,
                        images: None,
                    },
                    visibility: None,
                    raises: None,
                    subclasses: None,
                    read_type: Some(simple("double")),
                    write_type: Some(simple("double")),
                    optional: false,
                }],
            }),
        );
        let out = ts(&c);
        assert!(out.contains("pub struct DifficultySettings"));
        assert!(out.contains("pub technology_price_multiplier : f64"));
    }

    // --- sanitize_enum_variant ---

    #[test]
    fn variant_hyphenated() {
        assert_eq!(super::sanitize_enum_variant("top-left"), "TopLeft");
    }

    #[test]
    fn variant_keyword_escaped() {
        assert_eq!(super::sanitize_enum_variant("type"), "Type_");
    }

    #[test]
    fn variant_single_word() {
        assert_eq!(super::sanitize_enum_variant("left"), "Left");
    }
}
