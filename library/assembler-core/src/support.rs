use crate::concepts::LocalisedString;

/// Opaque handle to a Factorio Lua object, never constructed at runtime
#[derive(Clone, Debug)]
pub struct LuaHandle(());

/// Opaque Lua table, used when the structure is unknown or variable
#[derive(Clone, Debug)]
pub struct LuaTable;

/// Opaque Lua function value
#[derive(Clone, Copy, Debug)]
pub struct LuaFunction;

/// A Lua built-in primitive type (string, number, boolean, etc.)
#[derive(Clone, Copy, Debug)]
pub struct LuaBuiltin;

/// `any`, accepts any Lua value
#[derive(Clone, Copy, Debug)]
pub struct LuaAnyValue;

/// A Lua value that is one of many possible types (arity > 8)
/// Used as an escape hatch for large unions
#[derive(Clone, Copy, Debug)]
pub struct LuaMultiValue;

/// Lazy-loaded wrapper, the inner value is not fetched until accessed
#[derive(Clone, Copy, Debug)]
pub struct LuaLazyLoadedValue<T>(std::marker::PhantomData<T>);

/// A value that is one of 1 types (alias)
pub type Union1<A> = A;

/// A value that is one of 2 types
#[derive(Clone, Debug)]
pub enum Union2<A, B> {
    A(A),
    B(B),
}

impl<A, B> Union2<A, B> {
    pub fn a(val: A) -> Self {
        Self::A(val)
    }
    pub fn b(val: B) -> Self {
        Self::B(val)
    }
}

/// A value that is one of 3 types
#[derive(Clone, Debug)]
pub enum Union3<A, B, C> {
    A(A),
    B(B),
    C(C),
}

impl<A, B, C> Union3<A, B, C> {
    pub fn a(val: A) -> Self {
        Self::A(val)
    }
    pub fn b(val: B) -> Self {
        Self::B(val)
    }
    pub fn c(val: C) -> Self {
        Self::C(val)
    }
}

/// A value that is one of 4 types
#[derive(Clone, Debug)]
pub enum Union4<A, B, C, D> {
    A(A),
    B(B),
    C(C),
    D(D),
}

impl<A, B, C, D> Union4<A, B, C, D> {
    pub fn a(val: A) -> Self {
        Self::A(val)
    }
    pub fn b(val: B) -> Self {
        Self::B(val)
    }
    pub fn c(val: C) -> Self {
        Self::C(val)
    }
    pub fn d(val: D) -> Self {
        Self::D(val)
    }
}

/// A value that is one of 5 types
#[derive(Clone, Debug)]
pub enum Union5<A, B, C, D, E> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
}

impl<A, B, C, D, E> Union5<A, B, C, D, E> {
    pub fn a(val: A) -> Self {
        Self::A(val)
    }
    pub fn b(val: B) -> Self {
        Self::B(val)
    }
    pub fn c(val: C) -> Self {
        Self::C(val)
    }
    pub fn d(val: D) -> Self {
        Self::D(val)
    }
    pub fn e(val: E) -> Self {
        Self::E(val)
    }
}

/// A value that is one of 6 types
#[derive(Clone, Debug)]
pub enum Union6<A, B, C, D, E, F> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
    F(F),
}

/// A value that is one of 7 types
#[derive(Clone, Debug)]
pub enum Union7<A, B, C, D, E, F, G> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
    F(F),
    G(G),
}

/// A value that is one of 8 types
#[derive(Clone, Debug)]
pub enum Union8<A, B, C, D, E, F, G, H> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
    F(F),
    G(G),
    H(H),
}

impl From<&str> for LocalisedString {
    fn from(_: &str) -> Self {
        LocalisedString(vec![])
    }
}

impl From<String> for LocalisedString {
    fn from(_: String) -> Self {
        LocalisedString(vec![])
    }
}

impl From<f64> for LocalisedString {
    fn from(_: f64) -> Self {
        LocalisedString(vec![])
    }
}

impl From<f32> for LocalisedString {
    fn from(_: f32) -> Self {
        LocalisedString(vec![])
    }
}

impl From<i32> for LocalisedString {
    fn from(_: i32) -> Self {
        LocalisedString(vec![])
    }
}

impl From<u32> for LocalisedString {
    fn from(_: u32) -> Self {
        LocalisedString(vec![])
    }
}

impl From<bool> for LocalisedString {
    fn from(_: bool) -> Self {
        LocalisedString(vec![])
    }
}

/// Array form: `["key", param1, param2, ...]`
impl From<Vec<LocalisedString>> for LocalisedString {
    fn from(_: Vec<LocalisedString>) -> Self {
        LocalisedString(vec![])
    }
}
