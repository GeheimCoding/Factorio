#[derive(serde::Deserialize)]
pub struct RocketSiloPrototype {
    base_: crate::prototypes::AssemblingMachinePrototype,
    active_energy_usage: crate::types::Energy,
    alarm_sound: crate::types::Sound,
    alarm_trigger: crate::types::TriggerEffect,
    arm_01_back_animation: crate::types::Animation,
    arm_02_right_animation: crate::types::Animation,
    arm_03_front_animation: crate::types::Animation,
    base_day_sprite: crate::types::Sprite,
    base_engine_light: crate::types::LightDefinition,
    base_front_frozen: crate::types::Sprite,
    base_front_sprite: crate::types::Sprite,
    base_frozen: crate::types::Sprite,
    base_light: crate::types::LightDefinition,
    base_night_sprite: crate::types::Sprite,
    cargo_station_parameters: crate::types::CargoStationParameters,
    clamps_off_sound: crate::types::Sound,
    clamps_off_trigger: crate::types::TriggerEffect,
    clamps_on_sound: crate::types::Sound,
    clamps_on_trigger: crate::types::TriggerEffect,
    door_back_frozen: crate::types::Sprite,
    door_back_open_offset: crate::types::Vector,
    door_back_sprite: crate::types::Sprite,
    door_front_frozen: crate::types::Sprite,
    door_front_open_offset: crate::types::Vector,
    door_front_sprite: crate::types::Sprite,
    door_opening_speed: f64,
    doors_sound: crate::types::Sound,
    doors_trigger: crate::types::TriggerEffect,
    hole_clipping_box: crate::types::BoundingBox,
    hole_frozen: crate::types::Sprite,
    hole_light_sprite: crate::types::Sprite,
    hole_sprite: crate::types::Sprite,
    lamp_energy_usage: crate::types::Energy,
    #[serde(default = "default_launch_to_space_platforms")]
    launch_to_space_platforms: bool,
    #[serde(default = "default_launch_wait_time")]
    launch_wait_time: u8,
    light_blinking_speed: f64,
    #[serde(default = "default_logistic_trash_inventory_size")]
    logistic_trash_inventory_size: crate::types::ItemStackIndex,
    quick_alarm_sound: crate::types::Sound,
    raise_rocket_sound: crate::types::Sound,
    raise_rocket_trigger: crate::types::TriggerEffect,
    red_lights_back_sprites: crate::types::Sprite,
    red_lights_front_sprites: crate::types::Sprite,
    #[serde(default = "default_render_not_in_network_icon")]
    render_not_in_network_icon: bool,
    rocket_entity: crate::types::EntityID,
    rocket_glow_overlay_sprite: crate::types::Sprite,
    rocket_parts_required: u32,
    // default: Value of `rocket_parts_required`
    rocket_parts_storage_cap: u32,
    rocket_quick_relaunch_start_offset: f64,
    #[serde(default = "default_rocket_rising_delay")]
    rocket_rising_delay: u8,
    rocket_shadow_overlay_sprite: crate::types::Sprite,
    #[serde(default = "default_rocket_supply_inventory_size")]
    rocket_supply_inventory_size: crate::types::ItemStackIndex,
    satellite_animation: crate::types::Animation,
    satellite_shadow_animation: crate::types::Animation,
    shadow_sprite: crate::types::Sprite,
    silo_fade_out_end_distance: f64,
    silo_fade_out_start_distance: f64,
    times_to_blink: u8,
    #[serde(default = "default_to_be_inserted_to_rocket_inventory_size")]
    to_be_inserted_to_rocket_inventory_size: crate::types::ItemStackIndex,
}
fn default_launch_to_space_platforms() -> bool {
    false
}
fn default_launch_wait_time() -> u8 {
    120
}
fn default_logistic_trash_inventory_size() -> crate::types::ItemStackIndex {
    0
}
fn default_render_not_in_network_icon() -> bool {
    true
}
fn default_rocket_rising_delay() -> u8 {
    30
}
fn default_rocket_supply_inventory_size() -> crate::types::ItemStackIndex {
    0
}
fn default_to_be_inserted_to_rocket_inventory_size() -> crate::types::ItemStackIndex {
    0
}
