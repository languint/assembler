#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeFrame {
    Instantly,
    CurrentTick,
    FutureTick,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct EventRaised {
    pub name: String,
    pub order: u64,
    pub description: String,
    pub timeframe: TimeFrame,
    pub optional: bool,
}
