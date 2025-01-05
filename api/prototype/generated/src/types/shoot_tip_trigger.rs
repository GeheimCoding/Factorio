pub struct ShootTipTrigger {
    target: ShootTipTriggerTarget,
    type_: String,
}
pub enum ShootTipTriggerTarget {
    Enemy,
    Entity,
}
