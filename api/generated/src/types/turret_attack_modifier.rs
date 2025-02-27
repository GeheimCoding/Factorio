#[derive(Debug, serde::Deserialize)]
pub struct TurretAttackModifier {
    #[serde(flatten)]
    base_: crate::types::BaseModifier,
    #[serde(default = "default_infer_icon")]
    infer_icon: bool,
    modifier: f64,
    turret_id: crate::types::EntityID,
    #[serde(default = "default_use_icon_overlay_constant")]
    use_icon_overlay_constant: bool,
}
fn default_infer_icon() -> bool {
    true
}
fn default_use_icon_overlay_constant() -> bool {
    true
}
