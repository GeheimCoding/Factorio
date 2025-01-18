#[derive(serde::Deserialize)]
pub struct GunSpeedModifier {
    base_: crate::types::BaseModifier,
    ammo_category: crate::types::AmmoCategoryID,
    infer_icon: bool,
    modifier: f64,
    #[serde(rename = "type")]
    type_: String,
    use_icon_overlay_constant: bool,
}
