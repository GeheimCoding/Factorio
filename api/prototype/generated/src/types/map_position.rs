#[derive(Debug, serde::Deserialize)]
pub enum MapPosition {
    #[serde(untagged)]
    MapPosition { x: f64, y: f64 },
    #[serde(untagged)]
    F64F64((f64, f64)),
}
