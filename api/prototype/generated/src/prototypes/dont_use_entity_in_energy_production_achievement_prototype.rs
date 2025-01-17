#[derive(serde::Deserialize)]
pub struct DontUseEntityInEnergyProductionAchievementPrototype {
    base_: crate::prototypes::AchievementPrototypeWithCondition,
    excluded: DontUseEntityInEnergyProductionAchievementPrototypeExcluded,
    included: DontUseEntityInEnergyProductionAchievementPrototypeIncluded,
    last_hour_only: bool,
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
