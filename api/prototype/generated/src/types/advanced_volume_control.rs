#[derive(serde::Deserialize)]
pub struct AdvancedVolumeControl {
    attenuation: crate::types::Fade,
    #[serde(default = "default_darkness_threshold")]
    darkness_threshold: f32,
    fades: crate::types::Fades,
}
fn default_darkness_threshold() -> f32 {
    0.0
}
