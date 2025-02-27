#[derive(Debug, serde::Deserialize)]
pub struct NothingModifier {
    #[serde(flatten)]
    base_: crate::types::BaseModifier,
    effect_description: Option<crate::types::LocalisedString>,
    #[serde(default = "default_use_icon_overlay_constant")]
    use_icon_overlay_constant: bool,
}
fn default_use_icon_overlay_constant() -> bool {
    false
}
