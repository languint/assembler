use assembler_schema::schema::prelude::RuntimeApiRoot;

const PROTOTYPE: &'static str = include_str!("./prototype-api.json");
const RUNTIME: &'static str = include_str!("./runtime-api.json");
fn main() {
    let runtime = serde_json::from_str::<RuntimeApiRoot>(RUNTIME).expect("Failed to parse json");

    println!("{runtime:?}");
}
