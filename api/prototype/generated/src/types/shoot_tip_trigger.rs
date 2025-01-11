pub struct ShootTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    target: ShootTipTriggerTarget,
    type_: String,
}
pub enum ShootTipTriggerTarget {
    Enemy,
    Entity,
}
