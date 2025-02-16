use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct EventRaised {
    pub name: String,
    pub order: u16,
    pub description: String,
    pub timeframe: String,
    pub optional: bool,
}
