#[derive(Debug, serde::Deserialize)]
pub struct PlumeEffect {
    #[serde(flatten)]
    base_: crate::types::StatelessVisualisation,
    #[serde(default = "default_age_discrimination")]
    age_discrimination: i8,
}
fn default_age_discrimination() -> i8 {
    0
}
