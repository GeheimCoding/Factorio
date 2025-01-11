pub struct CorpsePrototype {
    animation: crate::types::RotatedAnimationVariations,
    animation_overlay: crate::types::RotatedAnimationVariations,
    animation_overlay_final_render_layer: crate::types::RenderLayer,
    animation_overlay_render_layer: crate::types::RenderLayer,
    animation_render_layer: crate::types::RenderLayer,
    decay_animation: crate::types::RotatedAnimationVariations,
    decay_frame_transition_duration: f32,
    direction_shuffle: Vec<Vec<u16>>,
    dying_speed: f32,
    expires: bool,
    final_render_layer: crate::types::RenderLayer,
    ground_patch: crate::types::AnimationVariations,
    ground_patch_decay: crate::types::AnimationVariations,
    ground_patch_fade_in_delay: f32,
    ground_patch_fade_in_speed: f32,
    ground_patch_fade_out_duration: f32,
    ground_patch_fade_out_start: f32,
    ground_patch_higher: crate::types::AnimationVariations,
    ground_patch_render_layer: crate::types::RenderLayer,
    remove_on_entity_placement: bool,
    remove_on_tile_placement: bool,
    shuffle_directions_at_frame: u8,
    splash: crate::types::AnimationVariations,
    splash_render_layer: crate::types::RenderLayer,
    splash_speed: f32,
    time_before_removed: u32,
    time_before_shading_off: u32,
    underwater_layer_offset: i8,
    underwater_patch: crate::types::RotatedSprite,
    use_decay_layer: bool,
    use_tile_color_for_ground_patch_tint: bool,
}