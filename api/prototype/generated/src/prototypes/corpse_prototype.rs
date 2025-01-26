#[derive(Debug, serde::Deserialize)]
pub struct CorpsePrototype {
    base_: crate::prototypes::EntityPrototype,
    animation: Option<crate::types::RotatedAnimationVariations>,
    animation_overlay: Option<crate::types::RotatedAnimationVariations>,
    #[serde(default = "default_animation_overlay_final_render_layer")]
    animation_overlay_final_render_layer: crate::types::RenderLayer,
    #[serde(default = "default_animation_overlay_render_layer")]
    animation_overlay_render_layer: crate::types::RenderLayer,
    #[serde(default = "default_animation_render_layer")]
    animation_render_layer: crate::types::RenderLayer,
    decay_animation: Option<crate::types::RotatedAnimationVariations>,
    #[serde(default = "default_decay_frame_transition_duration")]
    decay_frame_transition_duration: f32,
    direction_shuffle: Option<Vec<Vec<u16>>>,
    #[serde(default = "default_dying_speed")]
    dying_speed: f32,
    #[serde(default = "default_expires")]
    expires: bool,
    #[serde(default = "default_final_render_layer")]
    final_render_layer: crate::types::RenderLayer,
    ground_patch: Option<crate::types::AnimationVariations>,
    ground_patch_decay: Option<crate::types::AnimationVariations>,
    #[serde(default = "default_ground_patch_fade_in_delay")]
    ground_patch_fade_in_delay: f32,
    #[serde(default = "default_ground_patch_fade_in_speed")]
    ground_patch_fade_in_speed: f32,
    #[serde(default = "default_ground_patch_fade_out_duration")]
    ground_patch_fade_out_duration: f32,
    #[serde(default = "default_ground_patch_fade_out_start")]
    ground_patch_fade_out_start: f32,
    ground_patch_higher: Option<crate::types::AnimationVariations>,
    #[serde(default = "default_ground_patch_render_layer")]
    ground_patch_render_layer: crate::types::RenderLayer,
    #[serde(default = "default_remove_on_entity_placement")]
    remove_on_entity_placement: bool,
    #[serde(default = "default_remove_on_tile_placement")]
    remove_on_tile_placement: bool,
    #[serde(default = "default_shuffle_directions_at_frame")]
    shuffle_directions_at_frame: u8,
    splash: Option<crate::types::AnimationVariations>,
    #[serde(default = "default_splash_render_layer")]
    splash_render_layer: crate::types::RenderLayer,
    #[serde(default = "default_splash_speed")]
    splash_speed: f32,
    // default: 60 * 120 (120 seconds)
    time_before_removed: Option<u32>,
    // default: 60 * 15 (15 seconds)
    time_before_shading_off: Option<u32>,
    #[serde(default = "default_underwater_layer_offset")]
    underwater_layer_offset: i8,
    underwater_patch: Option<crate::types::RotatedSprite>,
    #[serde(default = "default_use_decay_layer")]
    use_decay_layer: bool,
    #[serde(default = "default_use_tile_color_for_ground_patch_tint")]
    use_tile_color_for_ground_patch_tint: bool,
}
fn default_animation_overlay_final_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Corpse
}
fn default_animation_overlay_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_animation_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_decay_frame_transition_duration() -> f32 {
    0.0
}
fn default_dying_speed() -> f32 {
    1.0
}
fn default_expires() -> bool {
    true
}
fn default_final_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Corpse
}
fn default_ground_patch_fade_in_delay() -> f32 {
    0.0
}
fn default_ground_patch_fade_in_speed() -> f32 {
    0.0
}
fn default_ground_patch_fade_out_duration() -> f32 {
    0.0
}
fn default_ground_patch_fade_out_start() -> f32 {
    0.0
}
fn default_ground_patch_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::GroundPatch
}
fn default_remove_on_entity_placement() -> bool {
    true
}
fn default_remove_on_tile_placement() -> bool {
    true
}
fn default_shuffle_directions_at_frame() -> u8 {
    1
}
fn default_splash_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_splash_speed() -> f32 {
    1.0
}
fn default_underwater_layer_offset() -> i8 {
    1
}
fn default_use_decay_layer() -> bool {
    false
}
fn default_use_tile_color_for_ground_patch_tint() -> bool {
    false
}
