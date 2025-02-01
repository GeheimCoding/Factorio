#[derive(Debug, serde::Deserialize)]
pub struct CharacterMiningSpeedModifier {
    base_: crate::types::SimpleModifier,
    #[serde(default = "default_use_icon_overlay_constant")]
    use_icon_overlay_constant: bool,
}
fn default_use_icon_overlay_constant() -> bool {
    true
}
