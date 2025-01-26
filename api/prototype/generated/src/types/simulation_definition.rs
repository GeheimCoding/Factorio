#[derive(Debug, serde::Deserialize)]
pub struct SimulationDefinition {
    #[serde(default = "default_checkboard")]
    checkboard: bool,
    game_view_settings: Option<crate::types::GameViewSettings>,
    #[serde(default = "default_generate_map")]
    generate_map: bool,
    hide_factoriopedia_gradient: Option<bool>,
    #[serde(default = "default_hide_health_bars")]
    hide_health_bars: bool,
    #[serde(default = "default_init")]
    init: String,
    init_file: Option<crate::types::FileName>,
    #[serde(default = "default_init_update_count")]
    init_update_count: u32,
    #[serde(default = "default_length")]
    length: u32,
    mods: Option<Vec<String>>,
    #[serde(default = "default_mute_alert_sounds")]
    mute_alert_sounds: bool,
    #[serde(default = "default_mute_technology_finished_sound")]
    mute_technology_finished_sound: bool,
    mute_wind_sounds: Option<bool>,
    override_volume: Option<bool>,
    #[serde(default = "default_planet")]
    planet: crate::types::SpaceLocationID,
    save: Option<crate::types::FileName>,
    #[serde(default = "default_update")]
    update: String,
    update_file: Option<crate::types::FileName>,
    volume_modifier: Option<f32>,
}
fn default_checkboard() -> bool {
    true
}
fn default_generate_map() -> bool {
    false
}
fn default_hide_health_bars() -> bool {
    true
}
fn default_init() -> String {
    String::from("")
}
fn default_init_update_count() -> u32 {
    0
}
fn default_length() -> u32 {
    0
}
fn default_mute_alert_sounds() -> bool {
    true
}
fn default_mute_technology_finished_sound() -> bool {
    true
}
fn default_planet() -> crate::types::SpaceLocationID {
    String::from("nauvis")
}
fn default_update() -> String {
    String::from("")
}
