use assembler_schema::prelude::*;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::emit::sanitize_ident;
use crate::emit::types::{map_param_type, map_type};

pub fn emit_class_shell(class: &Class) -> proc_macro2::TokenStream {
    let name = format_ident!("{}", class.basic_member.name);
    let doc = &class.basic_member.description;

    quote! {
        #[doc = #doc]
        #[derive(Debug, Clone)]
        pub struct #name(LuaHandle);
    }
}

pub fn emit_class_trait(class: &Class) -> TokenStream {
    let trait_name = trait_ident(&class.basic_member.name);
    let doc = &class.basic_member.description;

    // if this class has a parent, the trait extends it
    let supertrait = match &class.parent {
        Some(parent) => {
            let parent_trait = trait_ident(parent);
            quote! { : #parent_trait }
        }
        None => quote! { : crate::__Sealed },
    };

    let methods = class.methods.iter().map(emit_trait_method);
    let getters = class.attributes.iter().map(|a| emit_trait_getter(a, class));
    let setters = class
        .attributes
        .iter()
        .filter_map(|a| emit_trait_setter(a, class));
    let operators = class.operators.iter().map(emit_trait_operator);

    quote! {
        #[doc = #doc]
        #[doc(hidden)]
        pub trait #trait_name #supertrait {
            #(#methods)*
            #(#getters)*
            #(#setters)*
            #(#operators)*
        }
    }
}

pub fn emit_class_impl(class: &Class, all_classes: &[Class]) -> TokenStream {
    let name = format_ident!("{}", class.basic_member.name);
    let doc = &class.basic_member.description;

    let struct_def = quote! {
        #[doc = #doc]
        #[derive(Debug, Clone)]
        pub struct #name(LuaHandle);

        impl crate::__Sealed for #name {}
    };

    let own_impl = emit_class_trait_impl(class, &class.basic_member.name);

    let ancestor_impls = ancestors(&class.basic_member.name, all_classes)
        .into_iter()
        .map(|ancestor_name| {
            let ancestor = all_classes
                .iter()
                .find(|c| c.basic_member.name == ancestor_name)
                .expect("ancestor class should exist");
            emit_ancestor_trait_impl(class, ancestor)
        });

    quote! {
        #struct_def
        #own_impl
        #(#ancestor_impls)*
    }
}

fn emit_trait_method(method: &Method) -> TokenStream {
    let name = format_ident!("{}", sanitize_ident(&method.basic_member.name));
    let doc = &method.basic_member.description;
    let params = emit_method_params(method);
    let ret = emit_return_type(&method.return_values);

    quote! {
        #[doc = #doc]
        fn #name(&self, #(#params),*) -> #ret;
    }
}

fn emit_trait_getter(attr: &Attribute, class: &Class) -> TokenStream {
    let Some(read_type) = &attr.read_type else {
        return quote! {};
    };

    let base_name = sanitize_ident(&attr.basic_member.name);

    let has_operator_collision = class.operators.iter().any(|op| {
        matches!(
            (op, base_name.as_str()),
            (Operator::Index(_), "index") | (Operator::Length(_), "length")
        )
    });

    let name = if has_operator_collision {
        format_ident!("{}_attr", base_name)
    } else {
        format_ident!("{}", base_name)
    };

    let doc = &attr.basic_member.description;
    let ty = map_type(read_type);
    let ret = if attr.optional {
        quote! { Option<#ty> }
    } else {
        quote! { #ty }
    };

    quote! {
        #[doc = #doc]
        fn #name(&self) -> #ret;
    }
}

fn emit_trait_setter(attr: &Attribute, class: &Class) -> Option<TokenStream> {
    let write_type = attr.write_type.as_ref()?;

    let setter_name = format!("set_{}", sanitize_ident(&attr.basic_member.name));

    if class
        .methods
        .iter()
        .any(|m| m.basic_member.name == setter_name)
    {
        return None;
    }

    let setter_ident = format_ident!("{}", setter_name);
    let doc = format!("Set `{}`.", attr.basic_member.name);
    let ty = map_type(write_type);
    let param_ty = if attr.optional {
        quote! { Option<#ty> }
    } else {
        quote! { #ty }
    };

    Some(quote! {
        #[doc = #doc]
        fn #setter_ident(&mut self, value: #param_ty);
    })
}

fn emit_trait_operator(op: &Operator) -> TokenStream {
    match op {
        Operator::Call(method) => {
            let params = emit_method_params(method);
            let ret = emit_return_type(&method.return_values);
            let doc = &method.basic_member.description;
            quote! {
                #[doc = #doc]
                fn call(&self, #(#params),*) -> #ret;
            }
        }
        Operator::Index(attr) => {
            let doc = &attr.basic_member.description;
            let ty = attr
                .read_type
                .as_ref()
                .map(map_type)
                .unwrap_or(quote! { LuaAnyValue });
            let ret = if attr.optional {
                quote! { Option<#ty> }
            } else {
                quote! { #ty }
            };
            quote! {
                #[doc = #doc]
                fn index(&self, index: u64) -> #ret;
            }
        }
        Operator::Length(attr) => {
            let doc = &attr.basic_member.description;
            quote! {
                #[doc = #doc]
                fn length(&self) -> u64;
            }
        }
    }
}

fn emit_class_trait_impl(class: &Class, trait_class_name: &str) -> TokenStream {
    let struct_name = format_ident!("{}", class.basic_member.name);
    let trait_name = trait_ident(trait_class_name);

    let methods = class.methods.iter().map(emit_impl_method);
    let getters = class.attributes.iter().map(|a| emit_impl_getter(a, class));
    let setters = class
        .attributes
        .iter()
        .filter_map(|a| emit_impl_setter(a, class));
    let operators = class.operators.iter().map(emit_impl_operator);

    quote! {
        impl #trait_name for #struct_name {
            #(#methods)*
            #(#getters)*
            #(#setters)*
            #(#operators)*
        }
    }
}

fn emit_ancestor_trait_impl(child_class: &Class, ancestor: &Class) -> TokenStream {
    let child_name = format_ident!("{}", child_class.basic_member.name);
    let trait_name = trait_ident(&ancestor.basic_member.name);

    let methods = ancestor.methods.iter().map(emit_impl_method);
    let getters = ancestor
        .attributes
        .iter()
        .map(|a| emit_impl_getter(a, ancestor));
    let setters = ancestor
        .attributes
        .iter()
        .filter_map(|a| emit_impl_setter(a, ancestor));
    let operators = ancestor.operators.iter().map(emit_impl_operator);

    quote! {
        impl #trait_name for #child_name {
            #(#methods)*
            #(#getters)*
            #(#setters)*
            #(#operators)*
        }
    }
}

fn emit_impl_method(method: &Method) -> TokenStream {
    let name = format_ident!("{}", sanitize_ident(&method.basic_member.name));
    let params = emit_method_params(method);
    let ret = emit_return_type(&method.return_values);
    let lua = format!("`{}()`", method.basic_member.name);

    quote! {
        #[doc = #lua]
        fn #name(&self, #(#params),*) -> #ret {
            unreachable!("transpiled to Lua")
        }
    }
}

fn emit_impl_getter(attr: &Attribute, class: &Class) -> TokenStream {
    let Some(read_type) = &attr.read_type else {
        return quote! {};
    };

    let base_name = sanitize_ident(&attr.basic_member.name);

    let has_operator_collision = class.operators.iter().any(|op| {
        matches!(
            (op, base_name.as_str()),
            (Operator::Index(_), "index") | (Operator::Length(_), "length")
        )
    });

    let name = if has_operator_collision {
        format_ident!("{}_attr", base_name)
    } else {
        format_ident!("{}", base_name)
    };

    let ty = map_type(read_type);
    let ret = if attr.optional {
        quote! { Option<#ty> }
    } else {
        quote! { #ty }
    };

    quote! {
        fn #name(&self) -> #ret {
            unreachable!("transpiled to Lua")
        }
    }
}

fn emit_impl_setter(attr: &Attribute, class: &Class) -> Option<TokenStream> {
    let write_type = attr.write_type.as_ref()?;
    let setter_name = format!("set_{}", sanitize_ident(&attr.basic_member.name));

    if class
        .methods
        .iter()
        .any(|m| m.basic_member.name == setter_name)
    {
        return None;
    }

    let setter_ident = format_ident!("{}", setter_name);
    let ty = map_type(write_type);
    let param_ty = if attr.optional {
        quote! { Option<#ty> }
    } else {
        quote! { #ty }
    };

    Some(quote! {
        fn #setter_ident(&mut self, value: #param_ty) {
            unreachable!("transpiled to Lua")
        }
    })
}

fn emit_impl_operator(op: &Operator) -> TokenStream {
    match op {
        Operator::Call(method) => {
            let params = emit_method_params(method);
            let ret = emit_return_type(&method.return_values);
            quote! {
                fn call(&self, #(#params),*) -> #ret {
                    unreachable!("transpiled to Lua")
                }
            }
        }
        Operator::Index(attr) => {
            let ty = attr
                .read_type
                .as_ref()
                .map(map_type)
                .unwrap_or(quote! { LuaAnyValue });
            let ret = if attr.optional {
                quote! { Option<#ty> }
            } else {
                quote! { #ty }
            };
            quote! {
                fn index(&self, index: u64) -> #ret {
                    unreachable!("transpiled to Lua")
                }
            }
        }
        Operator::Length(_) => quote! {
            fn length(&self) -> u64 {
                unreachable!("transpiled to Lua")
            }
        },
    }
}

fn emit_method_params(method: &Method) -> Vec<TokenStream> {
    let mut params: Vec<TokenStream> = method
        .parameters
        .iter()
        .map(|p| {
            let name = format_ident!("{}", sanitize_ident(&p.name));
            let ty = map_param_type(&p.ty, p.optional);
            quote! { #name: #ty }
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

    if method
        .variant_parameter_groups
        .as_ref()
        .is_some_and(|g| !g.is_empty())
    {
        params.push(quote! { extra: Option<LuaTable> });
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

fn trait_ident(class_name: &str) -> syn::Ident {
    format_ident!("Is{}", class_name)
}

fn ancestors(class_name: &str, all_classes: &[Class]) -> Vec<String> {
    let parent = all_classes
        .iter()
        .find(|c| c.basic_member.name == class_name)
        .and_then(|c| c.parent.as_deref());

    match parent {
        None => vec![],
        Some(p) => {
            let mut chain = ancestors(p, all_classes);
            chain.push(p.to_string());
            chain
        }
    }
}
