pub mod factorio_version;
pub mod mod_dependency;
pub mod mod_version;

pub use factorio_version::FactorioVersion;
pub use mod_dependency::ModDependency;
pub use mod_version::ModVersion;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ModInfo {
    /// The internal identifier of the mod, like `mod-name`
    pub name: String,
    /// The version of the mod
    pub version: ModVersion,

    #[serde(flatten)]
    pub metadata: ModInfoMetadata,

    /// The target version the mod is developed for, defaults to 0.12
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factorio_version: Option<FactorioVersion>,
    /// Mods that this mod depends on or is incompatible with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<ModDependency>>,

    #[serde(flatten)]
    pub requirements: ModInfoRequirements,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ModInfoMetadata {
    /// The human-readable title of the mod, must be no longer than 100 chars.
    pub title: String,
    /// The author(s) of the mod
    pub author: String,
    /// How the mod author can be contacted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
    /// Where the mod can be found on the internet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    /// A short description of what the mod does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ModInfoRequirements {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rail_bridges_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoiling_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freezing_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmented_units_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expansion_shaders_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_travel_required: Option<bool>,
}

#[cfg(test)]
mod tests {
    use crate::info::{ModInfo, ModInfoMetadata, ModInfoRequirements, ModVersion};

    #[test]
    fn mod_no_deps() {
        let mod_info = ModInfo {
            name: "mod-info".to_string(),
            version: ModVersion::from((1, 0, 0)),
            factorio_version: None,
            dependencies: None,
            metadata: ModInfoMetadata {
                title: "My Mod".to_string(),
                author: "author".to_string(),
                ..Default::default()
            },
            requirements: ModInfoRequirements {
                space_travel_required: Some(true),
                ..Default::default()
            },
        };

        let mod_info_str = r#"{"name":"mod-info","version":"1.0.0","title":"My Mod","author":"author","space_travel_required":true}"#;

        assert_eq!(
            serde_json::to_string(&mod_info).unwrap().trim(),
            mod_info_str
        );

        assert_eq!(
            serde_json::from_str::<ModInfo>(mod_info_str).unwrap(),
            mod_info
        );
    }
}
