pub enum AttackParameters {
    ProjectileAttackParameters(Box<crate::types::ProjectileAttackParameters>),
    BeamAttackParameters(Box<crate::types::BeamAttackParameters>),
    StreamAttackParameters(Box<crate::types::StreamAttackParameters>),
}
