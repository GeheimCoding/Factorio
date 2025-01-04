pub struct ShootTipTrigger {
    target: ShootTipTriggerTarget,
    type_: Shoot,
}
pub enum ShootTipTriggerTarget {
    Enemy,
    Entity,
}
