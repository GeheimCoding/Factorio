#[derive(serde::Deserialize)]
pub struct DontUseEntityInEnergyProductionAchievementPrototype {
    base_: crate::prototypes::AchievementPrototypeWithCondition,
    excluded: DontUseEntityInEnergyProductionAchievementPrototypeExcluded,
    included: Option<DontUseEntityInEnergyProductionAchievementPrototypeIncluded>,
    #[serde(default = "default_last_hour_only")]
    last_hour_only: bool,
    #[serde(default = "default_minimum_energy_produced")]
    minimum_energy_produced: crate::types::Energy,
}
#[derive(serde::Deserialize)]
pub enum DontUseEntityInEnergyProductionAchievementPrototypeExcluded {
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
    #[serde(untagged)]
    VecEntityID(Vec<crate::types::EntityID>),
}
#[derive(serde::Deserialize)]
pub enum DontUseEntityInEnergyProductionAchievementPrototypeIncluded {
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
    #[serde(untagged)]
    VecEntityID(Vec<crate::types::EntityID>),
}
fn default_last_hour_only() -> bool {
    false
}
fn default_minimum_energy_produced() -> crate::types::Energy {
    String::from("0J")
}
