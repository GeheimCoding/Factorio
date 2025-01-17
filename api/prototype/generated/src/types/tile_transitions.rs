#[derive(serde::Deserialize)]
pub struct TileTransitions {
    apply_effect_color_to_overlay: bool,
    apply_waving_effect_on_background_mask: bool,
    apply_waving_effect_on_masks: bool,
    auxiliary_effect_mask_enabled: bool,
    auxiliary_effect_mask_layout: crate::types::TileTransitionVariantLayout,
    auxiliary_effect_mask_spritesheet: crate::types::FileName,
    background_enabled: bool,
    background_layer_group: crate::types::TileRenderLayer,
    background_layer_offset: i8,
    background_layout: crate::types::TileTransitionVariantLayout,
    background_mask_enabled: bool,
    background_mask_layout: crate::types::TileTransitionVariantLayout,
    background_mask_spritesheet: crate::types::FileName,
    background_spritesheet: crate::types::FileName,
    double_side_variations_in_group: u8,
    double_side_weights: Vec<f32>,
    draw_background_layer_under_tiles: bool,
    draw_simple_outer_corner_over_diagonal: bool,
    effect_map_enabled: bool,
    effect_map_layout: crate::types::TileTransitionVariantLayout,
    effect_map_spritesheet: crate::types::FileName,
    inner_corner_weights: Vec<f32>,
    layout: crate::types::TileTransitionSpritesheetLayout,
    lightmap_enabled: bool,
    lightmap_layout: crate::types::TileTransitionVariantLayout,
    lightmap_spritesheet: crate::types::FileName,
    mask_enabled: bool,
    mask_layout: crate::types::TileTransitionVariantLayout,
    mask_spritesheet: crate::types::FileName,
    masked_background_layer_offset: i8,
    masked_overlay_layer_offset: i8,
    offset_background_layer_by_tile_layer: bool,
    outer_corner_weights: Vec<f32>,
    overlay_enabled: bool,
    overlay_layer_group: crate::types::TileRenderLayer,
    overlay_layer_offset: i8,
    overlay_layout: crate::types::TileTransitionVariantLayout,
    side_variations_in_group: u8,
    side_weights: Vec<f32>,
    spritesheet: crate::types::FileName,
    u_transition_weights: Vec<f32>,
    water_patch: crate::types::Sprite,
    waving_effect_time_scale: f32,
}
