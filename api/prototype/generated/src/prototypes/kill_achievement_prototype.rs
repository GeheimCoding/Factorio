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
#[derive(serde::Deserialize)]
pub enum KillAchievementPrototypeDamageDealer {
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
    #[serde(untagged)]
    VecEntityID(Vec<crate::types::EntityID>),
}
#[derive(serde::Deserialize)]
pub enum KillAchievementPrototypeToKill {
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
    #[serde(untagged)]
    VecEntityID(Vec<crate::types::EntityID>),
}
