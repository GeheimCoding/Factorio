pub struct KillAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    amount: u32,
    damage_dealer: KillAchievementPrototypeDamageDealer,
    damage_type: crate::types::DamageTypeID,
    in_vehicle: bool,
    personally: bool,
    to_kill: KillAchievementPrototypeToKill,
    type_to_kill: String,
}
pub enum KillAchievementPrototypeDamageDealer {
    EntityID(crate::types::EntityID),
    VecEntityID(Vec<crate::types::EntityID>),
}
pub enum KillAchievementPrototypeToKill {
    EntityID(crate::types::EntityID),
    VecEntityID(Vec<crate::types::EntityID>),
}
