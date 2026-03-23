use assembler_schema::prelude::*;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::emit::sanitize_ident;
use crate::emit::types::map_type;

pub fn emit_globals(global_objects: &[Parameter], global_functions: &[Method]) -> TokenStream {
    let objects = global_objects.iter().map(emit_global_object);
    let functions = global_functions.iter().map(emit_global_function);
    let builtins = emit_builtin_globals();

    quote! {
        use crate::traits::*;
        use crate::concepts::*;
        use crate::support::*;

        #(#objects)*
        #(#functions)*
        #builtins
    }
}

fn emit_global_object(param: &Parameter) -> TokenStream {
    let name = format_ident!("{}", sanitize_ident(&param.name));
    let doc = &param.description;
    let ty = map_type(&param.ty);

    quote! {
        #[doc = #doc]
        #[must_use]
        pub fn #name() -> &'static #ty {
            unreachable!("transpiled to Lua global")
        }
    }
}

fn emit_global_function(method: &Method) -> TokenStream {
    let name = format_ident!("{}", sanitize_ident(&method.basic_member.name));
    let doc = &method.basic_member.description;
    let params = emit_function_params(method);
    let ret = emit_return_type(&method.return_values);

    quote! {
        #[doc = #doc]
        #[must_use]
        pub fn #name(#(#params),*) -> #ret {
            unreachable!("transpiled to Lua global")
        }
    }
}

fn emit_builtin_globals() -> TokenStream {
    quote! {
        /// The serpent serialisation library, available as a global
        #[allow(clippy::all, dead_code)]
        pub mod serpent {
            use crate::support::*;

            #[must_use]
            pub fn block(value: LuaAnyValue) -> String {
                unreachable!("transpiled to Lua: serpent.block(...)")
            }

            #[must_use]
            pub fn line(value: LuaAnyValue) -> String {
                unreachable!("transpiled to Lua: serpent.line(...)")
            }

            #[must_use]
            pub fn dump(value: LuaAnyValue) -> String {
                unreachable!("transpiled to Lua: serpent.dump(...)")
            }
        }
    }
}

fn emit_function_params(method: &Method) -> Vec<TokenStream> {
    let mut params: Vec<TokenStream> = method
        .parameters
        .iter()
        .map(|p| {
            let name = format_ident!("{}", sanitize_ident(&p.name));
            let ty = map_type(&p.ty);
            if p.optional {
                quote! { #name: Option<#ty> }
            } else {
                quote! { #name: #ty }
            }
        })
        .collect();

    if let Some(vp) = &method.variadic_parameter {
        let ty = vp
            .ty
            .as_ref()
            .map(map_type)
            .unwrap_or(quote! { LuaAnyValue });
        params.push(quote! { rest: Vec<#ty> });
    }

    params
}

fn emit_return_type(return_values: &[Parameter]) -> TokenStream {
    match return_values {
        [] => quote! { () },
        [single] => {
            let ty = map_type(&single.ty);
            if single.optional {
                quote! { Option<#ty> }
            } else {
                quote! { #ty }
            }
        }
        multiple => {
            let types = multiple.iter().map(|rv| {
                let ty = map_type(&rv.ty);
                if rv.optional {
                    quote! { Option<#ty> }
                } else {
                    quote! { #ty }
                }
            });
            quote! { (#(#types),*) }
        }
    }
}
