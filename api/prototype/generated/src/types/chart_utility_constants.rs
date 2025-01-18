#[derive(serde::Deserialize)]
pub struct ChartUtilityConstants {
    artillery_range_color: crate::types::Color,
    blue_signal_color: crate::types::Color,
    chart_construction_robot_color: crate::types::Color,
    chart_deconstruct_tint: crate::types::Color,
    chart_delivery_to_me_logistic_robot_color: crate::types::Color,
    chart_logistic_robot_color: crate::types::Color,
    chart_mobile_construction_robot_color: crate::types::Color,
    chart_personal_construction_robot_color: crate::types::Color,
    chart_player_circle_size: f32,
    chart_train_stop_disabled_text_color: crate::types::Color,
    chart_train_stop_full_text_color: crate::types::Color,
    chart_train_stop_text_color: crate::types::Color,
    circuit_network_member_color: crate::types::Color,
    copper_wire_color: crate::types::Color,
    #[serde(default = "default_custom_tag_max_scale")]
    custom_tag_max_scale: f32,
    #[serde(default = "default_custom_tag_scale")]
    custom_tag_scale: f32,
    custom_tag_selected_overlay_tint: crate::types::Color,
    default_color_by_type: std::collections::HashMap<String, crate::types::Color>,
    default_enemy_color: crate::types::Color,
    default_enemy_territory_color: crate::types::Color,
    default_friendly_color: crate::types::Color,
    default_friendly_color_by_type: std::collections::HashMap<String, crate::types::Color>,
    disabled_switch_color: crate::types::Color,
    electric_line_minimum_absolute_width: f32,
    electric_line_width: f32,
    electric_power_pole_color: crate::types::Color,
    elevated_rail_color: crate::types::Color,
    enabled_switch_color: crate::types::Color,
    entity_ghost_color: crate::types::Color,
    explosion_visualization_duration: u32,
    green_signal_color: crate::types::Color,
    green_wire_color: crate::types::Color,
    rail_color: crate::types::Color,
    rail_ramp_color: crate::types::Color,
    red_signal_color: crate::types::Color,
    red_wire_color: crate::types::Color,
    resource_outline_selection_color: crate::types::Color,
    tile_ghost_color: crate::types::Color,
    train_current_path_outline_color: crate::types::Color,
    train_path_color: crate::types::Color,
    train_preview_path_outline_color: crate::types::Color,
    turret_range_color: crate::types::Color,
    vehicle_inner_color: crate::types::Color,
    vehicle_outer_color: crate::types::Color,
    vehicle_outer_color_selected: crate::types::Color,
    vehicle_wagon_connection_color: crate::types::Color,
    yellow_signal_color: crate::types::Color,
    zoom_threshold_to_draw_spider_path: f64,
}
fn default_custom_tag_max_scale() -> f32 {
    1.0
}
fn default_custom_tag_scale() -> f32 {
    1.0
}
