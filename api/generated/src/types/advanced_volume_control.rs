#[derive(Debug, serde::Deserialize)]
pub struct AdvancedVolumeControl {
    attenuation: Option<crate::types::Fade>,
    #[serde(default = "default_darkness_threshold")]
    darkness_threshold: f32,
    fades: Option<crate::types::Fades>,
}
fn default_darkness_threshold() -> f32 {
    0.0
}
