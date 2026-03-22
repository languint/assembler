use assembler_codegen::emit::{
    concepts::emit_concept, emit_class_impl, emit_class_trait, emit_define,
};
use assembler_schema::prelude::RuntimeApiRoot;
use quote::{format_ident, quote};
use std::{fs, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-env-changed=FACTORIO_HOME");

    let home =
        std::env::var("FACTORIO_HOME").expect("FACTORIO_HOME must be set to generate factorio-api");

    let runtime_path = PathBuf::from(&home)
        .join("doc-html")
        .join("runtime-api.json");
    println!("cargo:rerun-if-changed={}", runtime_path.display());

    let json = fs::read_to_string(&runtime_path)
        .unwrap_or_else(|e| panic!("Could not read {}: {e}", runtime_path.display()));

    let api: RuntimeApiRoot =
        serde_json::from_str(&json).expect("Failed to parse runtime-api.json");

    let defines = api.defines.iter().map(emit_define);
    let defines_file = quote! {
        #(#defines)*
    };
    write_formatted(&out_dir.join("defines.rs"), defines_file);

    let traits_tokens = api.classes.iter().map(emit_class_trait);
    let impl_tokens = api.classes.iter().map(|c| emit_class_impl(c, &api.classes));

    let classes_file = quote! {
        use crate::support::*;
        use crate::concepts::*;
        #(#traits_tokens)*
        #(#impl_tokens)*
    };
    write_formatted(&out_dir.join("classes.rs"), classes_file);

    let reexports = api.classes.iter().map(|c| {
        let name = format_ident!("{}", c.basic_member.name);
        quote! { pub use crate::traits::#name; }
    });
    let reexports_file = quote! { #(#reexports)* };
    write_formatted(&out_dir.join("reexports.rs"), reexports_file);

    let concepts = api.concepts.iter().map(emit_concept);
    let concepts_file = quote! {
        use crate::traits::*;
        use crate::support::*;
        #(#concepts)*
    };
    write_formatted(&out_dir.join("concepts.rs"), concepts_file);
}

fn write_formatted(path: &PathBuf, tokens: proc_macro2::TokenStream) {
    let syntax_tree = syn::parse2(tokens).expect("codegen produced invalid Rust");
    let formatted = prettyplease::unparse(&syntax_tree);
    fs::write(path, formatted)
        .unwrap_or_else(|e| panic!("Failed to write {}: {e}", path.display()));
}
