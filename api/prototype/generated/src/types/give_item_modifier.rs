#[derive(serde::Deserialize)]
pub struct GiveItemModifier {
    base_: crate::types::BaseModifier,
    #[serde(default = "default_count")]
    count: crate::types::ItemCountType,
    item: crate::types::ItemID,
    #[serde(default = "default_quality")]
    quality: crate::types::QualityID,
    #[serde(rename = "type")]
    type_: String,
    #[serde(default = "default_use_icon_overlay_constant")]
    use_icon_overlay_constant: bool,
}
fn default_count() -> crate::types::ItemCountType {
    1
}
fn default_quality() -> crate::types::QualityID {
    String::from("normal")
}
fn default_use_icon_overlay_constant() -> bool {
    false
}
