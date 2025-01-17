#[derive(serde::Deserialize)]
pub struct RoboportPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    base: crate::types::Sprite,
    base_animation: crate::types::Animation,
    base_patch: crate::types::Sprite,
    charge_approach_distance: f32,
    charging_distance: f32,
    charging_energy: crate::types::Energy,
    charging_offsets: Vec<crate::types::Vector>,
    charging_station_count: u32,
    charging_station_count_affected_by_quality: bool,
    charging_station_shift: crate::types::Vector,
    charging_threshold_distance: f32,
    circuit_connector: crate::types::CircuitConnectorDefinition,
    circuit_wire_max_distance: f64,
    close_door_trigger_effect: crate::types::TriggerEffect,
    construction_radius: f32,
    default_available_construction_output_signal: crate::types::SignalIDConnector,
    default_available_logistic_output_signal: crate::types::SignalIDConnector,
    default_roboports_output_signal: crate::types::SignalIDConnector,
    default_total_construction_output_signal: crate::types::SignalIDConnector,
    default_total_logistic_output_signal: crate::types::SignalIDConnector,
    door_animation_down: crate::types::Animation,
    door_animation_up: crate::types::Animation,
    draw_circuit_wires: bool,
    draw_construction_radius_visualization: bool,
    draw_copper_wires: bool,
    draw_logistic_radius_visualization: bool,
    energy_source: RoboportPrototypeEnergySource,
    energy_usage: crate::types::Energy,
    frozen_patch: crate::types::Sprite,
    logistics_connection_distance: f32,
    logistics_radius: f32,
    material_slots_count: crate::types::ItemStackIndex,
    max_logistic_slots: crate::types::LogisticFilterIndex,
    open_door_trigger_effect: crate::types::TriggerEffect,
    radar_range: u32,
    recharge_minimum: crate::types::Energy,
    recharging_animation: crate::types::Animation,
    recharging_light: crate::types::LightDefinition,
    request_to_open_door_timeout: u32,
    robot_limit: crate::types::ItemCountType,
    robot_slots_count: crate::types::ItemStackIndex,
    robot_vertical_acceleration: f32,
    robots_shrink_when_entering_and_exiting: bool,
    spawn_and_station_height: f32,
    spawn_and_station_shadow_height_offset: f32,
    stationing_offset: crate::types::Vector,
    stationing_render_layer_swap_height: f32,
}
#[derive(serde::Deserialize)]
pub enum RoboportPrototypeEnergySource {
    #[serde(untagged)]
    ElectricEnergySource(Box<crate::types::ElectricEnergySource>),
    #[serde(untagged)]
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}
