pub enum AttackParameters {
    ProjectileAttackParameters(Box<ProjectileAttackParameters>),
    BeamAttackParameters(Box<BeamAttackParameters>),
    StreamAttackParameters(Box<StreamAttackParameters>),
}
