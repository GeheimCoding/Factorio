#[derive(serde::Deserialize)]
pub enum EffectRelativeTo {
    #[serde(rename = "ground_origin")]
    GroundOrigin,
    #[serde(rename = "pod")]
    Pod,
    #[serde(rename = "spawn_origin")]
    SpawnOrigin,
}
