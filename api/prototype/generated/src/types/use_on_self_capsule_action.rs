#[derive(serde::Deserialize)]
pub struct UseOnSelfCapsuleAction {
    attack_parameters: crate::types::AttackParameters,
    type_: String,
    uses_stack: bool,
}
