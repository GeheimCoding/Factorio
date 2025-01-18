#[derive(serde::Deserialize)]
pub struct UnitAISettings {
    #[serde(default = "default_allow_try_return_to_spawner")]
    allow_try_return_to_spawner: bool,
    #[serde(default = "default_destroy_when_commands_fail")]
    destroy_when_commands_fail: bool,
    #[serde(default = "default_do_separation")]
    do_separation: bool,
    #[serde(default = "default_path_resolution_modifier")]
    path_resolution_modifier: i8,
    strafe_settings: crate::types::PrototypeStrafeSettings,
}
fn default_allow_try_return_to_spawner() -> bool {
    false
}
fn default_destroy_when_commands_fail() -> bool {
    false
}
fn default_do_separation() -> bool {
    true
}
fn default_path_resolution_modifier() -> i8 {
    0
}
