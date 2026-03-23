pub mod support;

mod private {
    pub trait Sealed {}
}
pub use private::Sealed as __Sealed;

#[allow(
    non_upper_case_globals,
    non_snake_case,
    non_camel_case_types,
    unused_imports,
    clippy::all
)]
pub mod defines {
    use crate::support::*;
    include!(concat!(env!("OUT_DIR"), "/defines.rs"));
}

#[allow(
    non_upper_case_globals,
    non_snake_case,
    non_camel_case_types,
    dead_code,
    unused_variables,
    unused,
    clippy::all
)]
pub mod concepts {
    use crate::support::*;
    use std::collections::HashMap;
    include!(concat!(env!("OUT_DIR"), "/concepts.rs"));
}

#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    non_snake_case,
    dead_code,
    unused_variables,
    unused,
    clippy::all
)]
pub mod traits {
    use crate::concepts::*;
    use crate::support::*;
    use std::collections::HashMap;
    include!(concat!(env!("OUT_DIR"), "/classes.rs"));
}

#[allow(
    non_upper_case_globals,
    non_snake_case,
    dead_code,
    unused_variables,
    clippy::all
)]
pub mod globals {
    include!(concat!(env!("OUT_DIR"), "/globals.rs"));
}

#[allow(
    non_upper_case_globals,
    non_snake_case,
    dead_code,
    unused_variables,
    clippy::all
)]
pub mod prelude {
    include!(concat!(env!("OUT_DIR"), "/reexports.rs"));
    pub use crate::globals::serpent;
    pub use crate::globals::*;

    #[doc(hidden)]
    pub use crate::traits::*;
}
