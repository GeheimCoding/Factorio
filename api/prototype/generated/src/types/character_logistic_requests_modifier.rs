#[derive(serde::Deserialize)]
pub struct CharacterLogisticRequestsModifier {
    base_: crate::types::BoolModifier,
    #[serde(rename = "type")]
    type_: String,
    use_icon_overlay_constant: bool,
}
