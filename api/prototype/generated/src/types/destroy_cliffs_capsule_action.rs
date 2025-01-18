#[derive(serde::Deserialize)]
pub struct DestroyCliffsCapsuleAction {
    attack_parameters: crate::types::AttackParameters,
    play_sound_on_failure: bool,
    radius: f32,
    timeout: u32,
    #[serde(rename = "type")]
    type_: String,
    uses_stack: bool,
}
