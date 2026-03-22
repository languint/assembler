use assembler_schema::prelude::*;
use quote::{format_ident, quote};

fn sanitize_ident(s: &str) -> String {
    let base = s.replace('-', "_").replace('.', "__");
    if is_keyword(&base) {
        format!("r#{base}")
    } else {
        base
    }
}

fn is_keyword(s: &str) -> bool {
    matches!(s, |"as"| "break"
        | "const"
        | "continue"
        | "crate"
        | "else"
        | "enum"
        | "extern"
        | "false"
        | "fn"
        | "for"
        | "if"
        | "impl"
        | "in"
        | "let"
        | "loop"
        | "match"
        | "mod"
        | "move"
        | "mut"
        | "pub"
        | "ref"
        | "return"
        | "self"
        | "Self"
        | "static"
        | "struct"
        | "super"
        | "trait"
        | "true"
        | "type"
        | "unsafe"
        | "use"
        | "where"
        | "while"
        | "abstract"
        | "become"
        | "box"
        | "do"
        | "final"
        | "macro"
        | "override"
        | "priv"
        | "typeof"
        | "unsized"
        | "virtual"
        | "yield"
        | "async"
        | "await"
        | "dyn"
        | "try")
}

pub fn emit_define(define: &Define) -> proc_macro2::TokenStream {
    let name = format_ident!("{}", sanitize_ident(&define.basic_member.name));
    let doc = &define.basic_member.description;

    match (&define.values, &define.subkeys) {
        (Some(values), _) => {
            let consts = values.iter().enumerate().map(|(i, v)| {
                let vname = format_ident!("{}", sanitize_ident(&v.name));
                let doc = &v.description;
                let idx = i as u64;
                quote! {
                    #[doc = #doc]
                    pub const #vname: u64 = #idx;
                }
            });
            quote! {
                #[doc = #doc]
                pub mod #name {
                    #(#consts)*
                }
            }
        }
        (None, Some(subkeys)) => {
            let children = subkeys.iter().map(emit_define);
            quote! {
                #[doc = #doc]
                pub mod #name {
                    #(#children)*
                }
            }
        }
        (None, None) => quote! {
            pub mod #name {}
        },
    }
}
