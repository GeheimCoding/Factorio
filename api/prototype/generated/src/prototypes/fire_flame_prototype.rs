#[derive(serde::Deserialize)]
pub struct FireFlamePrototype {
    base_: crate::prototypes::EntityPrototype,
    #[serde(default = "default_add_fuel_cooldown")]
    add_fuel_cooldown: u32,
    #[serde(default = "default_burnt_patch_alpha_default")]
    burnt_patch_alpha_default: f32,
    burnt_patch_alpha_variations: Option<Vec<TileAndAlpha>>,
    #[serde(default = "default_burnt_patch_lifetime")]
    burnt_patch_lifetime: u32,
    burnt_patch_pictures: Option<crate::types::SpriteVariations>,
    #[serde(default = "default_damage_multiplier_decrease_per_tick")]
    damage_multiplier_decrease_per_tick: f32,
    #[serde(default = "default_damage_multiplier_increase_per_added_fuel")]
    damage_multiplier_increase_per_added_fuel: f32,
    damage_per_tick: crate::types::DamageParameters,
    #[serde(default = "default_delay_between_initial_flames")]
    delay_between_initial_flames: u32,
    #[serde(default = "default_fade_in_duration")]
    fade_in_duration: u32,
    #[serde(default = "default_fade_out_duration")]
    fade_out_duration: u32,
    #[serde(default = "default_flame_alpha")]
    flame_alpha: f32,
    #[serde(default = "default_flame_alpha_deviation")]
    flame_alpha_deviation: f32,
    #[serde(default = "default_initial_flame_count")]
    initial_flame_count: u8,
    #[serde(default = "default_initial_lifetime")]
    initial_lifetime: u32,
    #[serde(default = "default_initial_render_layer")]
    initial_render_layer: crate::types::RenderLayer,
    #[serde(default = "default_lifetime_increase_by")]
    lifetime_increase_by: u32,
    #[serde(default = "default_lifetime_increase_cooldown")]
    lifetime_increase_cooldown: u32,
    light: Option<crate::types::LightDefinition>,
    #[serde(default = "default_light_size_modifier_maximum")]
    light_size_modifier_maximum: f32,
    #[serde(default = "default_light_size_modifier_per_flame")]
    light_size_modifier_per_flame: f32,
    #[serde(default = "default_limit_overlapping_particles")]
    limit_overlapping_particles: bool,
    #[serde(default = "default_maximum_damage_multiplier")]
    maximum_damage_multiplier: f32,
    // default: Max uint32
    maximum_lifetime: Option<u32>,
    #[serde(default = "default_maximum_spread_count")]
    maximum_spread_count: u16,
    on_damage_tick_effect: Option<crate::types::Trigger>,
    on_fuel_added_action: Option<crate::types::Trigger>,
    #[serde(default = "default_particle_alpha")]
    particle_alpha: f32,
    #[serde(default = "default_particle_alpha_blend_duration")]
    particle_alpha_blend_duration: u16,
    #[serde(default = "default_particle_alpha_deviation")]
    particle_alpha_deviation: f32,
    pictures: Option<crate::types::AnimationVariations>,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    #[serde(default = "default_secondary_picture_fade_out_duration")]
    secondary_picture_fade_out_duration: u32,
    #[serde(default = "default_secondary_picture_fade_out_start")]
    secondary_picture_fade_out_start: u32,
    secondary_pictures: Option<crate::types::AnimationVariations>,
    #[serde(default = "default_secondary_render_layer")]
    secondary_render_layer: crate::types::RenderLayer,
    small_tree_fire_pictures: Option<crate::types::AnimationVariations>,
    smoke: Option<Vec<crate::types::SmokeSource>>,
    #[serde(default = "default_smoke_fade_in_duration")]
    smoke_fade_in_duration: u32,
    #[serde(default = "default_smoke_fade_out_duration")]
    smoke_fade_out_duration: u32,
    smoke_source_pictures: Option<crate::types::AnimationVariations>,
    spawn_entity: Option<crate::types::EntityID>,
    spread_delay: u32,
    spread_delay_deviation: u32,
    #[serde(default = "default_tree_dying_factor")]
    tree_dying_factor: f32,
    #[serde(default = "default_uses_alternative_behavior")]
    uses_alternative_behavior: bool,
}
fn default_add_fuel_cooldown() -> u32 {
    10
}
fn default_burnt_patch_alpha_default() -> f32 {
    1.0
}
#[derive(serde::Deserialize)]
pub struct TileAndAlpha {
    alpha: f32,
    tile: crate::types::TileID,
}
fn default_burnt_patch_lifetime() -> u32 {
    1800
}
fn default_damage_multiplier_decrease_per_tick() -> f32 {
    0.0
}
fn default_damage_multiplier_increase_per_added_fuel() -> f32 {
    0.0
}
fn default_delay_between_initial_flames() -> u32 {
    10
}
fn default_fade_in_duration() -> u32 {
    30
}
fn default_fade_out_duration() -> u32 {
    30
}
fn default_flame_alpha() -> f32 {
    1.0
}
fn default_flame_alpha_deviation() -> f32 {
    0.0
}
fn default_initial_flame_count() -> u8 {
    0
}
fn default_initial_lifetime() -> u32 {
    300
}
fn default_initial_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_lifetime_increase_by() -> u32 {
    20
}
fn default_lifetime_increase_cooldown() -> u32 {
    10
}
fn default_light_size_modifier_maximum() -> f32 {
    1.0
}
fn default_light_size_modifier_per_flame() -> f32 {
    0.0
}
fn default_limit_overlapping_particles() -> bool {
    false
}
fn default_maximum_damage_multiplier() -> f32 {
    1.0
}
fn default_maximum_spread_count() -> u16 {
    200
}
fn default_particle_alpha() -> f32 {
    1.0
}
fn default_particle_alpha_blend_duration() -> u16 {
    0
}
fn default_particle_alpha_deviation() -> f32 {
    0.0
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_secondary_picture_fade_out_duration() -> u32 {
    30
}
fn default_secondary_picture_fade_out_start() -> u32 {
    0
}
fn default_secondary_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_smoke_fade_in_duration() -> u32 {
    30
}
fn default_smoke_fade_out_duration() -> u32 {
    30
}
fn default_tree_dying_factor() -> f32 {
    0.0
}
fn default_uses_alternative_behavior() -> bool {
    false
}
