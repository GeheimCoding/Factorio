#[derive(Debug, serde::Deserialize)]
pub struct ThrowCapsuleAction {
    attack_parameters: crate::types::AttackParameters,
    #[serde(default = "default_uses_stack")]
    uses_stack: bool,
}
fn default_uses_stack() -> bool {
    true
}
