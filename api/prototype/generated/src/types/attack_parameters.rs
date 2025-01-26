#[derive(Debug, serde::Deserialize)]
pub enum AttackParameters {
    #[serde(untagged)]
    ProjectileAttackParameters(Box<crate::types::ProjectileAttackParameters>),
    #[serde(untagged)]
    BeamAttackParameters(Box<crate::types::BeamAttackParameters>),
    #[serde(untagged)]
    StreamAttackParameters(Box<crate::types::StreamAttackParameters>),
}
