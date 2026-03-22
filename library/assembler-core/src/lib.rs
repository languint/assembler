#![allow(non_upper_case_globals, non_snake_case)]

/// Opaque handle to a Factorio Lua object, never constructed at runtime.
pub struct LuaHandle(());

pub mod defines {
    include!(concat!(env!("OUT_DIR"), "/defines.rs"));
}

pub mod classes {
    include!(concat!(env!("OUT_DIR"), "/classes.rs"));
}
