#[derive(serde::Deserialize)]
pub struct RocketSiloRocketPrototype {
    base_: crate::prototypes::EntityPrototype,
    cargo_attachment_offset: crate::types::Vector,
    cargo_pod_entity: crate::types::EntityID,
    dying_explosion: crate::types::EntityID,
    effects_fade_in_end_distance: f64,
    effects_fade_in_start_distance: f64,
    engine_starting_speed: f64,
    flying_acceleration: f64,
    flying_sound: crate::types::Sound,
    flying_speed: f64,
    flying_trigger: crate::types::TriggerEffect,
    full_render_layer_switch_distance: f64,
    glow_light: crate::types::LightDefinition,
    inventory_size: crate::types::ItemStackIndex,
    rising_speed: f64,
    #[serde(default = "default_rocket_above_wires_slice_offset_from_center")]
    rocket_above_wires_slice_offset_from_center: f32,
    #[serde(default = "default_rocket_air_object_slice_offset_from_center")]
    rocket_air_object_slice_offset_from_center: f32,
    rocket_flame_animation: crate::types::Animation,
    rocket_flame_left_animation: crate::types::Animation,
    rocket_flame_left_rotation: f32,
    rocket_flame_right_animation: crate::types::Animation,
    rocket_flame_right_rotation: f32,
    // default: `{rect={{-30, -30}, {30, rocket_above_wires_slice_offset_from_center}}, falloff=1}`
    rocket_fog_mask: crate::types::FogMaskShapeDefinition,
    rocket_glare_overlay_sprite: crate::types::Sprite,
    rocket_initial_offset: crate::types::Vector,
    rocket_launch_offset: crate::types::Vector,
    rocket_render_layer_switch_distance: f64,
    rocket_rise_offset: crate::types::Vector,
    rocket_shadow_sprite: crate::types::Sprite,
    #[serde(rename = "rocket_smoke_bottom1_animation")]
    rocket_smoke_bottom_1_animation: crate::types::Animation,
    #[serde(rename = "rocket_smoke_bottom2_animation")]
    rocket_smoke_bottom_2_animation: crate::types::Animation,
    #[serde(rename = "rocket_smoke_top1_animation")]
    rocket_smoke_top_1_animation: crate::types::Animation,
    #[serde(rename = "rocket_smoke_top2_animation")]
    rocket_smoke_top_2_animation: crate::types::Animation,
    #[serde(rename = "rocket_smoke_top3_animation")]
    rocket_smoke_top_3_animation: crate::types::Animation,
    rocket_sprite: crate::types::Sprite,
    rocket_visible_distance_from_center: f32,
    shadow_fade_out_end_ratio: f64,
    shadow_fade_out_start_ratio: f64,
    shadow_slave_entity: crate::types::EntityID,
}
fn default_rocket_above_wires_slice_offset_from_center() -> f32 {
    -3.0
}
fn default_rocket_air_object_slice_offset_from_center() -> f32 {
    -5.5
}
