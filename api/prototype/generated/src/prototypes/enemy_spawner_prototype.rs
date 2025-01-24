#[derive(serde::Deserialize)]
pub struct EnemySpawnerPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    absorptions_per_second: Option<
        std::collections::HashMap<
            crate::types::AirbornePollutantID,
            crate::types::EnemySpawnerAbsorption,
        >,
    >,
    #[serde(default = "default_allow_run_time_change_of_is_military_target")]
    allow_run_time_change_of_is_military_target: bool,
    call_for_help_radius: f64,
    captured_spawner_entity: Option<crate::types::EntityID>,
    dying_sound: Option<crate::types::Sound>,
    graphics_set: crate::types::EnemySpawnerGraphicsSet,
    #[serde(default = "default_is_military_target")]
    is_military_target: bool,
    max_count_of_owned_units: u32,
    #[serde(default = "default_max_darkness_to_spawn")]
    max_darkness_to_spawn: f32,
    max_friends_around_to_spawn: u32,
    max_richness_for_spawn_shift: f64,
    max_spawn_shift: f64,
    #[serde(default = "default_min_darkness_to_spawn")]
    min_darkness_to_spawn: f32,
    result_units: Vec<crate::types::UnitSpawnDefinition>,
    spawn_decoration: Option<Vec<crate::types::CreateDecorativesTriggerEffectItem>>,
    #[serde(default = "default_spawn_decorations_on_expansion")]
    spawn_decorations_on_expansion: bool,
    spawning_cooldown: (f64, f64),
    spawning_radius: f64,
    spawning_spacing: f64,
    #[serde(default = "default_time_to_capture")]
    time_to_capture: u32,
}
fn default_allow_run_time_change_of_is_military_target() -> bool {
    false
}
fn default_is_military_target() -> bool {
    true
}
fn default_max_darkness_to_spawn() -> f32 {
    1.0
}
fn default_min_darkness_to_spawn() -> f32 {
    0.0
}
fn default_spawn_decorations_on_expansion() -> bool {
    false
}
fn default_time_to_capture() -> u32 {
    0
}
