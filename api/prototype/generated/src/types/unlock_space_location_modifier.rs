#[derive(serde::Deserialize)]
pub struct UnlockSpaceLocationModifier {
    base_: crate::types::BaseModifier,
    space_location: crate::types::SpaceLocationID,
    #[serde(rename = "type")]
    type_: String,
    use_icon_overlay_constant: bool,
}
