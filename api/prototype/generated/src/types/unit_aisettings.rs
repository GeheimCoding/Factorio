#[derive(serde::Deserialize)]
pub struct UnitAISettings {
    allow_try_return_to_spawner: bool,
    destroy_when_commands_fail: bool,
    do_separation: bool,
    path_resolution_modifier: i8,
    strafe_settings: crate::types::PrototypeStrafeSettings,
}
