#[derive(serde::Deserialize)]
pub enum Vector {
    #[serde(untagged)]
    Vector { x: f64, y: f64 },
    #[serde(untagged)]
    F64F64((f64, f64)),
}
