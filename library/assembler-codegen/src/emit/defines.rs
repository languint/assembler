use assembler_schema::prelude::*;
use quote::{format_ident, quote};

use crate::emit::sanitize_ident;

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
