#[derive(serde::Deserialize)]
pub struct SpiderLegPart {
    bottom_end: Option<crate::types::RotatedSprite>,
    #[serde(default = "default_bottom_end_length")]
    bottom_end_length: f32,
    #[serde(default = "default_bottom_end_offset")]
    bottom_end_offset: f32,
    middle: Option<crate::types::RotatedSprite>,
    #[serde(default = "default_middle_offset_from_bottom")]
    middle_offset_from_bottom: f32,
    #[serde(default = "default_middle_offset_from_top")]
    middle_offset_from_top: f32,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    top_end: Option<crate::types::RotatedSprite>,
    #[serde(default = "default_top_end_length")]
    top_end_length: f32,
    #[serde(default = "default_top_end_offset")]
    top_end_offset: f32,
}
fn default_bottom_end_length() -> f32 {
    0.0
}
fn default_bottom_end_offset() -> f32 {
    0.0
}
fn default_middle_offset_from_bottom() -> f32 {
    0.0
}
fn default_middle_offset_from_top() -> f32 {
    0.0
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::HigherObjectAbove
}
fn default_top_end_length() -> f32 {
    0.0
}
fn default_top_end_offset() -> f32 {
    0.0
}
