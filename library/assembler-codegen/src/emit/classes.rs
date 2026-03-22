use assembler_schema::prelude::*;
use quote::{format_ident, quote};

pub fn emit_class_shell(class: &Class) -> proc_macro2::TokenStream {
    let name = format_ident!("{}", class.basic_member.name);
    let doc = &class.basic_member.description;

    quote! {
        #[doc = #doc]
        #[derive(Debug, Clone)]
        pub struct #name(LuaHandle);
    }
}
