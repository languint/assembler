#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Image {
    pub filename: String,
    pub caption: Option<String>,
}
