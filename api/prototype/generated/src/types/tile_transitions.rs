#[derive(serde::Deserialize)]
pub struct TileTransitions {
    #[serde(default = "default_apply_effect_color_to_overlay")]
    apply_effect_color_to_overlay: bool,
    // default: Value of `apply_waving_effect_on_masks`
    apply_waving_effect_on_background_mask: Option<bool>,
    #[serde(default = "default_apply_waving_effect_on_masks")]
    apply_waving_effect_on_masks: bool,
    #[serde(default = "default_auxiliary_effect_mask_enabled")]
    auxiliary_effect_mask_enabled: bool,
    auxiliary_effect_mask_layout: Option<crate::types::TileTransitionVariantLayout>,
    // default: Value of `spritesheet`
    auxiliary_effect_mask_spritesheet: Option<crate::types::FileName>,
    #[serde(default = "default_background_enabled")]
    background_enabled: bool,
    background_layer_group: Option<crate::types::TileRenderLayer>,
    #[serde(default = "default_background_layer_offset")]
    background_layer_offset: i8,
    background_layout: Option<crate::types::TileTransitionVariantLayout>,
    #[serde(default = "default_background_mask_enabled")]
    background_mask_enabled: bool,
    background_mask_layout: Option<crate::types::TileTransitionVariantLayout>,
    // default: Value of `spritesheet`
    background_mask_spritesheet: Option<crate::types::FileName>,
    // default: Value of `spritesheet`
    background_spritesheet: Option<crate::types::FileName>,
    double_side_variations_in_group: Option<u8>,
    double_side_weights: Option<Vec<f32>>,
    #[serde(default = "default_draw_background_layer_under_tiles")]
    draw_background_layer_under_tiles: bool,
    #[serde(default = "default_draw_simple_outer_corner_over_diagonal")]
    draw_simple_outer_corner_over_diagonal: bool,
    #[serde(default = "default_effect_map_enabled")]
    effect_map_enabled: bool,
    effect_map_layout: Option<crate::types::TileTransitionVariantLayout>,
    // default: Value of `spritesheet`
    effect_map_spritesheet: Option<crate::types::FileName>,
    inner_corner_weights: Option<Vec<f32>>,
    layout: Option<crate::types::TileTransitionSpritesheetLayout>,
    #[serde(default = "default_lightmap_enabled")]
    lightmap_enabled: bool,
    lightmap_layout: Option<crate::types::TileTransitionVariantLayout>,
    // default: Value of `spritesheet`
    lightmap_spritesheet: Option<crate::types::FileName>,
    #[serde(default = "default_mask_enabled")]
    mask_enabled: bool,
    mask_layout: Option<crate::types::TileTransitionVariantLayout>,
    // default: Value of `spritesheet`
    mask_spritesheet: Option<crate::types::FileName>,
    // default: Value of `background_layer_offset`
    masked_background_layer_offset: Option<i8>,
    #[serde(default = "default_masked_overlay_layer_offset")]
    masked_overlay_layer_offset: i8,
    #[serde(default = "default_offset_background_layer_by_tile_layer")]
    offset_background_layer_by_tile_layer: bool,
    outer_corner_weights: Option<Vec<f32>>,
    #[serde(default = "default_overlay_enabled")]
    overlay_enabled: bool,
    overlay_layer_group: Option<crate::types::TileRenderLayer>,
    overlay_layer_offset: Option<i8>,
    overlay_layout: Option<crate::types::TileTransitionVariantLayout>,
    side_variations_in_group: Option<u8>,
    side_weights: Option<Vec<f32>>,
    spritesheet: Option<crate::types::FileName>,
    u_transition_weights: Option<Vec<f32>>,
    water_patch: Option<crate::types::Sprite>,
    #[serde(default = "default_waving_effect_time_scale")]
    waving_effect_time_scale: f32,
}
fn default_apply_effect_color_to_overlay() -> bool {
    false
}
fn default_apply_waving_effect_on_masks() -> bool {
    false
}
fn default_auxiliary_effect_mask_enabled() -> bool {
    true
}
fn default_background_enabled() -> bool {
    true
}
fn default_background_layer_offset() -> i8 {
    0
}
fn default_background_mask_enabled() -> bool {
    true
}
fn default_draw_background_layer_under_tiles() -> bool {
    false
}
fn default_draw_simple_outer_corner_over_diagonal() -> bool {
    true
}
fn default_effect_map_enabled() -> bool {
    true
}
fn default_lightmap_enabled() -> bool {
    true
}
fn default_mask_enabled() -> bool {
    true
}
fn default_masked_overlay_layer_offset() -> i8 {
    0
}
fn default_offset_background_layer_by_tile_layer() -> bool {
    false
}
fn default_overlay_enabled() -> bool {
    true
}
fn default_waving_effect_time_scale() -> f32 {
    0.1
}
