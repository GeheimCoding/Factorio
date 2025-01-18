#[derive(serde::Deserialize)]
pub struct UseOnSelfCapsuleAction {
    attack_parameters: crate::types::AttackParameters,
    #[serde(rename = "type")]
    type_: String,
    uses_stack: bool,
}
