#[derive(Debug, serde::Deserialize)]
pub enum NoiseExpression {
    #[serde(untagged)]
    String(String),
    #[serde(untagged)]
    Bool(bool),
    #[serde(untagged)]
    F64(f64),
}
