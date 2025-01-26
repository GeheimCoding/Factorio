#[derive(Debug, serde::Deserialize)]
pub struct UnlockSpaceLocationModifier {
    base_: crate::types::BaseModifier,
    space_location: crate::types::SpaceLocationID,
    #[serde(rename = "type")]
    type_: String,
    #[serde(default = "default_use_icon_overlay_constant")]
    use_icon_overlay_constant: bool,
}
fn default_use_icon_overlay_constant() -> bool {
    false
}
