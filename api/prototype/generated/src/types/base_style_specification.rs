#[derive(Debug, serde::Deserialize)]
pub struct BaseStyleSpecification {
    #[serde(default = "default_bottom_margin")]
    bottom_margin: i16,
    #[serde(default = "default_bottom_padding")]
    bottom_padding: i16,
    effect: Option<BaseStyleSpecificationEffect>,
    #[serde(default = "default_effect_opacity")]
    effect_opacity: f32,
    height: Option<u32>,
    #[serde(default = "default_horizontal_align")]
    horizontal_align: crate::types::HorizontalAlign,
    #[serde(default = "default_horizontally_squashable")]
    horizontally_squashable: crate::types::StretchRule,
    #[serde(default = "default_horizontally_stretchable")]
    horizontally_stretchable: crate::types::StretchRule,
    ignored_by_search: Option<bool>,
    #[serde(default = "default_left_margin")]
    left_margin: i16,
    #[serde(default = "default_left_padding")]
    left_padding: i16,
    margin: Option<i16>,
    #[serde(default = "default_maximal_height")]
    maximal_height: u32,
    #[serde(default = "default_maximal_width")]
    maximal_width: u32,
    #[serde(default = "default_minimal_height")]
    minimal_height: u32,
    #[serde(default = "default_minimal_width")]
    minimal_width: u32,
    #[serde(default = "default_natural_height")]
    natural_height: u32,
    natural_size: Option<BaseStyleSpecificationNaturalSize>,
    #[serde(default = "default_natural_width")]
    natural_width: u32,
    never_hide_by_search: Option<bool>,
    padding: Option<i16>,
    parent: Option<String>,
    #[serde(default = "default_right_margin")]
    right_margin: i16,
    #[serde(default = "default_right_padding")]
    right_padding: i16,
    size: Option<BaseStyleSpecificationSize>,
    tooltip: Option<crate::types::LocalisedString>,
    #[serde(default = "default_top_margin")]
    top_margin: i16,
    #[serde(default = "default_top_padding")]
    top_padding: i16,
    #[serde(default = "default_vertical_align")]
    vertical_align: crate::types::VerticalAlign,
    #[serde(default = "default_vertically_squashable")]
    vertically_squashable: crate::types::StretchRule,
    #[serde(default = "default_vertically_stretchable")]
    vertically_stretchable: crate::types::StretchRule,
    width: Option<u32>,
}
fn default_bottom_margin() -> i16 {
    0
}
fn default_bottom_padding() -> i16 {
    0
}
#[derive(Debug, serde::Deserialize)]
pub enum BaseStyleSpecificationEffect {
    #[serde(rename = "compilatron_hologram")]
    CompilatronHologram,
}
fn default_effect_opacity() -> f32 {
    1.0
}
fn default_horizontal_align() -> crate::types::HorizontalAlign {
    crate::types::HorizontalAlign::Left
}
fn default_horizontally_squashable() -> crate::types::StretchRule {
    crate::types::StretchRule::Auto
}
fn default_horizontally_stretchable() -> crate::types::StretchRule {
    crate::types::StretchRule::Auto
}
fn default_left_margin() -> i16 {
    0
}
fn default_left_padding() -> i16 {
    0
}
fn default_maximal_height() -> u32 {
    0
}
fn default_maximal_width() -> u32 {
    0
}
fn default_minimal_height() -> u32 {
    0
}
fn default_minimal_width() -> u32 {
    0
}
fn default_natural_height() -> u32 {
    0
}
#[derive(Debug, serde::Deserialize)]
pub enum BaseStyleSpecificationNaturalSize {
    #[serde(untagged)]
    U32(u32),
    #[serde(untagged)]
    U32U32((u32, u32)),
}
fn default_natural_width() -> u32 {
    0
}
fn default_right_margin() -> i16 {
    0
}
fn default_right_padding() -> i16 {
    0
}
#[derive(Debug, serde::Deserialize)]
pub enum BaseStyleSpecificationSize {
    #[serde(untagged)]
    U32(u32),
    #[serde(untagged)]
    U32U32((u32, u32)),
}
fn default_top_margin() -> i16 {
    0
}
fn default_top_padding() -> i16 {
    0
}
fn default_vertical_align() -> crate::types::VerticalAlign {
    crate::types::VerticalAlign::Top
}
fn default_vertically_squashable() -> crate::types::StretchRule {
    crate::types::StretchRule::Auto
}
fn default_vertically_stretchable() -> crate::types::StretchRule {
    crate::types::StretchRule::Auto
}
