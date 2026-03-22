#[allow(non_upper_case_globals, non_snake_case)]
pub mod defines {
    include!(concat!(env!("OUT_DIR"), "/defines.rs"));
}
