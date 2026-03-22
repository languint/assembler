use std::path::PathBuf;

pub fn factorio_doc(filename: &str) -> Option<String> {
    let home = std::env::var("FACTORIO_HOME").ok()?;
    let path = PathBuf::from(home).join("doc-html").join(filename);
    Some(std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "FACTORIO_HOME is set but could not read {}: {e}",
            path.display()
        )
    }))
}

pub fn runtime_json() -> Option<String> {
    factorio_doc("runtime-api.json")
}

pub fn prototype_json() -> Option<String> {
    factorio_doc("prototype-api.json")
}

pub fn de<T: serde::de::DeserializeOwned>(json: &str) -> T {
    serde_json::from_str(json)
        .unwrap_or_else(|e| panic!("Deserialization failed: {e}\n\nInput JSON:\n{json}"))
}

#[track_caller]
pub fn assert_de_fails<T: serde::de::DeserializeOwned + std::fmt::Debug>(json: &str) {
    assert!(
        serde_json::from_str::<T>(json).is_err(),
        "Expected deserialization to fail but it succeeded for: {json}"
    );
}
