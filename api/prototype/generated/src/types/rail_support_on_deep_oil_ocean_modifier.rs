#[derive(serde::Deserialize)]
pub struct RailSupportOnDeepOilOceanModifier {
    base_: crate::types::BoolModifier,
    #[serde(rename = "type")]
    type_: String,
    #[serde(default = "default_use_icon_overlay_constant")]
    use_icon_overlay_constant: bool,
}
fn default_use_icon_overlay_constant() -> bool {
    false
}
