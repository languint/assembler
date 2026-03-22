#[derive(Debug, Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeFrame {
    Instantly,
    CurrentTick,
    FutureTick,
}
