#[derive(Debug, serde::Deserialize)]
pub struct StickerPrototype {
    base_: crate::prototypes::EntityPrototype,
    animation: Option<crate::types::Animation>,
    #[serde(default = "default_damage_interval")]
    damage_interval: u32,
    damage_per_tick: Option<crate::types::DamageParameters>,
    duration_in_ticks: u32,
    #[serde(default = "default_fire_spread_cooldown")]
    fire_spread_cooldown: u8,
    #[serde(default = "default_fire_spread_radius")]
    fire_spread_radius: f32,
    #[serde(default = "default_force_visibility")]
    force_visibility: crate::types::ForceCondition,
    #[serde(default = "default_ground_target")]
    ground_target: bool,
    #[serde(default = "default_hidden")]
    hidden: bool,
    // default: Value of `hidden`
    hidden_in_factoriopedia: Option<bool>,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    #[serde(default = "default_selection_box_type")]
    selection_box_type: crate::types::CursorBoxType,
    #[serde(default = "default_single_particle")]
    single_particle: bool,
    spread_fire_entity: Option<crate::types::EntityID>,
    #[serde(default = "default_stickers_per_square_meter")]
    stickers_per_square_meter: f32,
    #[serde(default = "default_target_movement_max")]
    target_movement_max: f32,
    // default: Value of `target_movement_speed`
    target_movement_max_from: Option<f32>,
    // default: Value of `target_movement_speed`
    target_movement_max_to: Option<f32>,
    #[serde(default = "default_target_movement_modifier")]
    target_movement_modifier: f32,
    // default: Value of `target_movement_modifier`
    target_movement_modifier_from: Option<f32>,
    // default: Value of `target_movement_modifier`
    target_movement_modifier_to: Option<f32>,
    update_effects: Option<Vec<crate::types::TriggerEffectWithCooldown>>,
    #[serde(default = "default_vehicle_friction_modifier")]
    vehicle_friction_modifier: f32,
    // default: Value of `vehicle_friction_modifier`
    vehicle_friction_modifier_from: Option<f32>,
    // default: Value of `vehicle_friction_modifier`
    vehicle_friction_modifier_to: Option<f32>,
    #[serde(default = "default_vehicle_speed_max")]
    vehicle_speed_max: f32,
    // default: Value of `vehicle_speed_max`
    vehicle_speed_max_from: Option<f32>,
    // default: Value of `vehicle_speed_max`
    vehicle_speed_max_to: Option<f32>,
    #[serde(default = "default_vehicle_speed_modifier")]
    vehicle_speed_modifier: f32,
    // default: Value of `vehicle_speed_modifier`
    vehicle_speed_modifier_from: Option<f32>,
    // default: Value of `vehicle_speed_modifier`
    vehicle_speed_modifier_to: Option<f32>,
}
fn default_damage_interval() -> u32 {
    1
}
fn default_fire_spread_cooldown() -> u8 {
    30
}
fn default_fire_spread_radius() -> f32 {
    1.0
}
fn default_force_visibility() -> crate::types::ForceCondition {
    crate::types::ForceCondition::All
}
fn default_ground_target() -> bool {
    false
}
fn default_hidden() -> bool {
    true
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_selection_box_type() -> crate::types::CursorBoxType {
    crate::types::CursorBoxType::Entity
}
fn default_single_particle() -> bool {
    false
}
fn default_stickers_per_square_meter() -> f32 {
    15.0
}
fn default_target_movement_max() -> f32 {
    -1.0
}
fn default_target_movement_modifier() -> f32 {
    1.0
}
fn default_vehicle_friction_modifier() -> f32 {
    1.0
}
fn default_vehicle_speed_max() -> f32 {
    -1.0
}
fn default_vehicle_speed_modifier() -> f32 {
    1.0
}
