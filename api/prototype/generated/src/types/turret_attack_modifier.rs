#[derive(serde::Deserialize)]
pub struct TurretAttackModifier {
    base_: crate::types::BaseModifier,
    infer_icon: bool,
    modifier: f64,
    turret_id: crate::types::EntityID,
    type_: String,
    use_icon_overlay_constant: bool,
}
