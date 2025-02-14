#[derive(Debug, serde::Deserialize)]
pub struct DestroyCliffsCapsuleAction {
    attack_parameters: crate::types::AttackParameters,
    #[serde(default = "default_play_sound_on_failure")]
    play_sound_on_failure: bool,
    radius: f32,
    #[serde(default = "default_timeout")]
    timeout: u32,
    #[serde(default = "default_uses_stack")]
    uses_stack: bool,
}
fn default_play_sound_on_failure() -> bool {
    true
}
fn default_timeout() -> u32 {
    3600
}
fn default_uses_stack() -> bool {
    true
}
