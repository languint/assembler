pub mod classes;
pub mod concepts;
pub mod defines;
pub mod types;

pub use classes::{emit_class_impl, emit_class_shell, emit_class_trait};
pub use defines::emit_define;

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
