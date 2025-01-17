#[derive(serde::Deserialize)]
pub enum AmmoSourceType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "player")]
    Player,
    #[serde(rename = "turret")]
    Turret,
    #[serde(rename = "vehicle")]
    Vehicle,
}
