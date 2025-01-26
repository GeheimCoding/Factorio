#[derive(Debug, serde::Deserialize)]
pub struct UnlockQualityModifier {
    base_: crate::types::BaseModifier,
    quality: crate::types::QualityID,
    #[serde(rename = "type")]
    type_: String,
    #[serde(default = "default_use_icon_overlay_constant")]
    use_icon_overlay_constant: bool,
}
fn default_use_icon_overlay_constant() -> bool {
    false
}
