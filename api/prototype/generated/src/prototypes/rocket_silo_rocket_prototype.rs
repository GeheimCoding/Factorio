#[derive(Debug, serde::Deserialize)]
pub struct RocketSiloRocketPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityPrototype,
    cargo_attachment_offset: Option<crate::types::Vector>,
    cargo_pod_entity: crate::types::EntityID,
    dying_explosion: Option<crate::types::EntityID>,
    effects_fade_in_end_distance: f64,
    effects_fade_in_start_distance: f64,
    engine_starting_speed: f64,
    flying_acceleration: f64,
    flying_sound: Option<crate::types::Sound>,
    flying_speed: f64,
    flying_trigger: Option<crate::types::TriggerEffect>,
    full_render_layer_switch_distance: f64,
    glow_light: Option<crate::types::LightDefinition>,
    inventory_size: crate::types::ItemStackIndex,
    rising_speed: f64,
    #[serde(default = "default_rocket_above_wires_slice_offset_from_center")]
    rocket_above_wires_slice_offset_from_center: f32,
    #[serde(default = "default_rocket_air_object_slice_offset_from_center")]
    rocket_air_object_slice_offset_from_center: f32,
    rocket_flame_animation: Option<crate::types::Animation>,
    rocket_flame_left_animation: Option<crate::types::Animation>,
    rocket_flame_left_rotation: f32,
    rocket_flame_right_animation: Option<crate::types::Animation>,
    rocket_flame_right_rotation: f32,
    // default: `{rect={{-30, -30}, {30, rocket_above_wires_slice_offset_from_center}}, falloff=1}`
    rocket_fog_mask: Option<crate::types::FogMaskShapeDefinition>,
    rocket_glare_overlay_sprite: Option<crate::types::Sprite>,
    rocket_initial_offset: Option<crate::types::Vector>,
    rocket_launch_offset: crate::types::Vector,
    rocket_render_layer_switch_distance: f64,
    rocket_rise_offset: crate::types::Vector,
    rocket_shadow_sprite: Option<crate::types::Sprite>,
    #[serde(rename = "rocket_smoke_bottom1_animation")]
    rocket_smoke_bottom_1_animation: Option<crate::types::Animation>,
    #[serde(rename = "rocket_smoke_bottom2_animation")]
    rocket_smoke_bottom_2_animation: Option<crate::types::Animation>,
    #[serde(rename = "rocket_smoke_top1_animation")]
    rocket_smoke_top_1_animation: Option<crate::types::Animation>,
    #[serde(rename = "rocket_smoke_top2_animation")]
    rocket_smoke_top_2_animation: Option<crate::types::Animation>,
    #[serde(rename = "rocket_smoke_top3_animation")]
    rocket_smoke_top_3_animation: Option<crate::types::Animation>,
    rocket_sprite: Option<crate::types::Sprite>,
    rocket_visible_distance_from_center: f32,
    shadow_fade_out_end_ratio: f64,
    shadow_fade_out_start_ratio: f64,
    shadow_slave_entity: Option<crate::types::EntityID>,
}
fn default_rocket_above_wires_slice_offset_from_center() -> f32 {
    -3.0
}
fn default_rocket_air_object_slice_offset_from_center() -> f32 {
    -5.5
}
