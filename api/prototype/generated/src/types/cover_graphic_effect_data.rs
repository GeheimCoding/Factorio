#[derive(serde::Deserialize)]
pub struct CoverGraphicEffectData {
    // default: {0,0}
    distance_traveled_strength: Option<crate::types::Vector>,
    // default: {0,0}
    pod_movement_strength: Option<crate::types::Vector>,
    #[serde(default = "default_relative_to")]
    relative_to: crate::types::EffectRelativeTo,
    #[serde(default = "default_style")]
    style: crate::types::CloudEffectStyle,
}
fn default_relative_to() -> crate::types::EffectRelativeTo {
    crate::types::EffectRelativeTo::Pod
}
fn default_style() -> crate::types::CloudEffectStyle {
    crate::types::CloudEffectStyle::None
}
