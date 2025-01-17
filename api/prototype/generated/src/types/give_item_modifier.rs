#[derive(serde::Deserialize)]
pub struct GiveItemModifier {
    base_: crate::types::BaseModifier,
    count: crate::types::ItemCountType,
    item: crate::types::ItemID,
    quality: crate::types::QualityID,
    type_: String,
    use_icon_overlay_constant: bool,
}
