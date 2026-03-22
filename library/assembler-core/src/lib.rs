pub mod support;

#[allow(
    non_upper_case_globals,
    non_snake_case,
    non_camel_case_types,
    unused_imports
)]
pub mod defines {
    use crate::support::*;
    include!(concat!(env!("OUT_DIR"), "/defines.rs"));
}

#[allow(non_upper_case_globals, non_snake_case, dead_code)]
pub mod classes {
    use crate::support::*;
    include!(concat!(env!("OUT_DIR"), "/classes.rs"));
}

#[allow(
    non_upper_case_globals,
    non_snake_case,
    non_camel_case_types,
    dead_code
)]
pub mod concepts {
    use crate::support::*;
    include!(concat!(env!("OUT_DIR"), "/concepts.rs"));
}
