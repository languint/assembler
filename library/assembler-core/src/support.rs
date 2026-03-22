/// Opaque handle to a Factorio Lua object, never constructed at runtime
#[derive(Clone, Debug)]
pub struct LuaHandle(());

/// Opaque Lua table, used when the structure is unknown or variable
#[derive(Clone, Debug)]
pub struct LuaTable;

/// Opaque Lua function value
#[derive(Clone, Debug)]
pub struct LuaFunction;

/// A Lua built-in primitive type (string, number, boolean, etc.)
#[derive(Clone, Debug)]
pub struct LuaBuiltin;

/// `any`, accepts any Lua value
#[derive(Clone, Debug)]
pub struct LuaAnyValue;

/// A Lua value that is one of many possible types (arity > 8)
#[derive(Clone, Debug)]
pub struct LuaMultiValue;

/// Lazy-loaded wrapper, the inner value is not fetched until accessed
#[derive(Clone, Debug)]
pub struct LuaLazyLoadedValue<T>(std::marker::PhantomData<T>);

/// Union types, used when a field can be one of N types
/// Never constructed at runtime, transpiled to Lua type checks
pub type Union1<A> = A;
#[derive(Clone, Debug)]
pub struct Union2<A, B>(std::marker::PhantomData<(A, B)>);
#[derive(Clone, Debug)]
pub struct Union3<A, B, C>(std::marker::PhantomData<(A, B, C)>);
#[derive(Clone, Debug)]
pub struct Union4<A, B, C, D>(std::marker::PhantomData<(A, B, C, D)>);
#[derive(Clone, Debug)]
pub struct Union5<A, B, C, D, E>(std::marker::PhantomData<(A, B, C, D, E)>);
#[derive(Clone, Debug)]
pub struct Union6<A, B, C, D, E, F>(std::marker::PhantomData<(A, B, C, D, E, F)>);
#[derive(Clone, Debug)]
pub struct Union7<A, B, C, D, E, F, G>(std::marker::PhantomData<(A, B, C, D, E, F, G)>);
#[derive(Clone, Debug)]
pub struct Union8<A, B, C, D, E, F, G, H>(std::marker::PhantomData<(A, B, C, D, E, F, G, H)>);
