#[derive(serde::Deserialize)]
pub struct NothingModifier {
    base_: crate::types::BaseModifier,
    effect_description: crate::types::LocalisedString,
    #[serde(rename = "type")]
    type_: String,
    #[serde(default = "default_use_icon_overlay_constant")]
    use_icon_overlay_constant: bool,
}
fn default_use_icon_overlay_constant() -> bool {
    false
}
