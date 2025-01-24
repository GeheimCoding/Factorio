#[derive(serde::Deserialize)]
pub struct KillAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    #[serde(default = "default_amount")]
    amount: u32,
    damage_dealer: Option<KillAchievementPrototypeDamageDealer>,
    damage_type: Option<crate::types::DamageTypeID>,
    #[serde(default = "default_in_vehicle")]
    in_vehicle: bool,
    #[serde(default = "default_personally")]
    personally: bool,
    to_kill: Option<KillAchievementPrototypeToKill>,
    type_to_kill: Option<String>,
}
fn default_amount() -> u32 {
    1
}
#[derive(serde::Deserialize)]
pub enum KillAchievementPrototypeDamageDealer {
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
    #[serde(untagged)]
    VecEntityID(Vec<crate::types::EntityID>),
}
fn default_in_vehicle() -> bool {
    false
}
fn default_personally() -> bool {
    false
}
#[derive(serde::Deserialize)]
pub enum KillAchievementPrototypeToKill {
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
    #[serde(untagged)]
    VecEntityID(Vec<crate::types::EntityID>),
}
