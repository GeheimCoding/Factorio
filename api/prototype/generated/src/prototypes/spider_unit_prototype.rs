#[derive(serde::Deserialize)]
pub struct SpiderUnitPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    absorptions_to_join_attack: std::collections::HashMap<crate::types::AirbornePollutantID, f32>,
    ai_settings: crate::types::UnitAISettings,
    attack_parameters: crate::types::AttackParameters,
    distraction_cooldown: u32,
    dying_sound: crate::types::Sound,
    graphics_set: crate::types::SpiderTorsoGraphicsSet,
    height: f32,
    #[serde(default = "default_max_pursue_distance")]
    max_pursue_distance: f64,
    #[serde(default = "default_min_pursue_time")]
    min_pursue_time: u32,
    #[serde(default = "default_radar_range")]
    radar_range: u32,
    #[serde(default = "default_spawning_time_modifier")]
    spawning_time_modifier: f64,
    spider_engine: crate::types::SpiderEngineSpecification,
    #[serde(default = "default_torso_bob_speed")]
    torso_bob_speed: f32,
    #[serde(default = "default_torso_rotation_speed")]
    torso_rotation_speed: f32,
    vision_distance: f64,
    warcry: crate::types::Sound,
}
fn default_max_pursue_distance() -> f64 {
    50.0
}
fn default_min_pursue_time() -> u32 {
    600
}
fn default_radar_range() -> u32 {
    0
}
fn default_spawning_time_modifier() -> f64 {
    1.0
}
fn default_torso_bob_speed() -> f32 {
    1.0
}
fn default_torso_rotation_speed() -> f32 {
    1.0
}
