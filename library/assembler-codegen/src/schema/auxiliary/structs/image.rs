#[derive(Debug, Clone, serde::Deserialize)]
pub struct Image {
    pub filename: String,
    pub caption: String,
}
