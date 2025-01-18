#[derive(serde::Deserialize)]
pub struct RoboportEquipmentPrototype {
    base_: crate::prototypes::EquipmentPrototype,
    burner: crate::types::BurnerEnergySource,
    charge_approach_distance: f32,
    #[serde(default = "default_charging_distance")]
    charging_distance: f32,
    charging_energy: crate::types::Energy,
    charging_offsets: Vec<crate::types::Vector>,
    #[serde(default = "default_charging_station_count")]
    charging_station_count: u32,
    #[serde(default = "default_charging_station_count_affected_by_quality")]
    charging_station_count_affected_by_quality: bool,
    charging_station_shift: crate::types::Vector,
    #[serde(default = "default_charging_threshold_distance")]
    charging_threshold_distance: f32,
    construction_radius: f32,
    #[serde(default = "default_draw_construction_radius_visualization")]
    draw_construction_radius_visualization: bool,
    #[serde(default = "default_draw_logistic_radius_visualization")]
    draw_logistic_radius_visualization: bool,
    power: crate::types::Energy,
    recharging_animation: crate::types::Animation,
    recharging_light: crate::types::LightDefinition,
    // default: max uint
    robot_limit: crate::types::ItemCountType,
    #[serde(default = "default_robot_vertical_acceleration")]
    robot_vertical_acceleration: f32,
    #[serde(default = "default_robots_shrink_when_entering_and_exiting")]
    robots_shrink_when_entering_and_exiting: bool,
    spawn_and_station_height: f32,
    #[serde(default = "default_spawn_and_station_shadow_height_offset")]
    spawn_and_station_shadow_height_offset: f32,
    // default: 0.2 * energy_source.buffer_capacity
    spawn_minimum: crate::types::Energy,
    stationing_offset: crate::types::Vector,
    #[serde(default = "default_stationing_render_layer_swap_height")]
    stationing_render_layer_swap_height: f32,
}
fn default_charging_distance() -> f32 {
    0.0
}
fn default_charging_station_count() -> u32 {
    0
}
fn default_charging_station_count_affected_by_quality() -> bool {
    false
}
fn default_charging_threshold_distance() -> f32 {
    1.0
}
fn default_draw_construction_radius_visualization() -> bool {
    true
}
fn default_draw_logistic_radius_visualization() -> bool {
    true
}
fn default_robot_vertical_acceleration() -> f32 {
    0.0
}
fn default_robots_shrink_when_entering_and_exiting() -> bool {
    false
}
fn default_spawn_and_station_shadow_height_offset() -> f32 {
    0.0
}
fn default_stationing_render_layer_swap_height() -> f32 {
    0.9
}
