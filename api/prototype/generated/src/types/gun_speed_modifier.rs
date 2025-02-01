#[derive(Debug, serde::Deserialize)]
pub struct GunSpeedModifier {
    base_: crate::types::BaseModifier,
    ammo_category: crate::types::AmmoCategoryID,
    #[serde(default = "default_infer_icon")]
    infer_icon: bool,
    modifier: f64,
    #[serde(default = "default_use_icon_overlay_constant")]
    use_icon_overlay_constant: bool,
}
fn default_infer_icon() -> bool {
    true
}
fn default_use_icon_overlay_constant() -> bool {
    true
}
