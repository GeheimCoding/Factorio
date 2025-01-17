#[derive(serde::Deserialize)]
pub struct BaseStyleSpecification {
    bottom_margin: i16,
    bottom_padding: i16,
    effect: BaseStyleSpecificationEffect,
    effect_opacity: f32,
    height: u32,
    horizontal_align: crate::types::HorizontalAlign,
    horizontally_squashable: crate::types::StretchRule,
    horizontally_stretchable: crate::types::StretchRule,
    ignored_by_search: bool,
    left_margin: i16,
    left_padding: i16,
    margin: i16,
    maximal_height: u32,
    maximal_width: u32,
    minimal_height: u32,
    minimal_width: u32,
    natural_height: u32,
    natural_size: BaseStyleSpecificationNaturalSize,
    natural_width: u32,
    never_hide_by_search: bool,
    padding: i16,
    parent: String,
    right_margin: i16,
    right_padding: i16,
    size: BaseStyleSpecificationSize,
    tooltip: crate::types::LocalisedString,
    top_margin: i16,
    top_padding: i16,
    vertical_align: crate::types::VerticalAlign,
    vertically_squashable: crate::types::StretchRule,
    vertically_stretchable: crate::types::StretchRule,
    width: u32,
}
#[derive(serde::Deserialize)]
pub enum BaseStyleSpecificationEffect {
    #[serde(rename = "compilatron_hologram")]
    CompilatronHologram,
}
#[derive(serde::Deserialize)]
pub enum BaseStyleSpecificationNaturalSize {
    #[serde(untagged)]
    U32(u32),
    #[serde(untagged)]
    U32U32((u32, u32)),
}
#[derive(serde::Deserialize)]
pub enum BaseStyleSpecificationSize {
    #[serde(untagged)]
    U32(u32),
    #[serde(untagged)]
    U32U32((u32, u32)),
}
