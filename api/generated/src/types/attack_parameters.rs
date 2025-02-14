#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
pub enum AttackParameters {
    #[serde(rename = "projectile")]
    ProjectileAttackParameters(Box<crate::types::ProjectileAttackParameters>),
    #[serde(rename = "beam")]
    BeamAttackParameters(Box<crate::types::BeamAttackParameters>),
    #[serde(rename = "stream")]
    StreamAttackParameters(Box<crate::types::StreamAttackParameters>),
}
