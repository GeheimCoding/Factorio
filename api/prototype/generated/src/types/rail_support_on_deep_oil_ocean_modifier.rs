#[derive(Debug, serde::Deserialize)]
pub struct RailSupportOnDeepOilOceanModifier {
    #[serde(flatten)]
    base_: crate::types::BoolModifier,
    #[serde(default = "default_use_icon_overlay_constant")]
    use_icon_overlay_constant: bool,
}
fn default_use_icon_overlay_constant() -> bool {
    false
}
