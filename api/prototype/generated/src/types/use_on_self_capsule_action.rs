#[derive(Debug, serde::Deserialize)]
pub struct UseOnSelfCapsuleAction {
    attack_parameters: crate::types::AttackParameters,
    #[serde(rename = "type")]
    type_: String,
    #[serde(default = "default_uses_stack")]
    uses_stack: bool,
}
fn default_uses_stack() -> bool {
    true
}
