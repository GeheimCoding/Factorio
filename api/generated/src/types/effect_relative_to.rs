#[derive(Debug, serde::Deserialize)]
pub enum EffectRelativeTo {
    #[serde(rename = "ground-origin")]
    GroundOrigin,
    #[serde(rename = "pod")]
    Pod,
    #[serde(rename = "spawn-origin")]
    SpawnOrigin,
}
