#[derive(serde::Deserialize)]
pub struct CharacterLootPickupDistanceModifier {
    base_: crate::types::SimpleModifier,
    type_: String,
    use_icon_overlay_constant: bool,
}
