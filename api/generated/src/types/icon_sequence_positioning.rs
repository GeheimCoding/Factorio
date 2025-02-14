#[derive(Debug, serde::Deserialize)]
pub struct IconSequencePositioning {
    inventory_index: crate::defines::Inventory,
    // default: width of entity selection box / 1.5
    max_icon_rows: Option<u8>,
    // default: width of entity selection box / 0.75
    max_icons_per_row: Option<u8>,
    #[serde(default = "default_multi_row_initial_height_modifier")]
    multi_row_initial_height_modifier: f32,
    #[serde(default = "default_scale")]
    scale: f32,
    #[serde(default = "default_separation_multiplier")]
    separation_multiplier: f32,
    // default: {0, 0.7}
    shift: Option<crate::types::Vector>,
}
fn default_multi_row_initial_height_modifier() -> f32 {
    -0.1
}
fn default_scale() -> f32 {
    0.5
}
fn default_separation_multiplier() -> f32 {
    1.1
}
