#[derive(serde::Deserialize)]
pub struct UnlockQualityModifier {
    base_: crate::types::BaseModifier,
    quality: crate::types::QualityID,
    type_: String,
    use_icon_overlay_constant: bool,
}
