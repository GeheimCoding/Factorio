pub struct DontUseEntityInEnergyProductionAchievementPrototype {
    excluded: DontUseEntityInEnergyProductionAchievementPrototypeExcluded,
    included: DontUseEntityInEnergyProductionAchievementPrototypeIncluded,
    last_hour_only: bool,
    minimum_energy_produced: crate::types::Energy,
}
pub enum DontUseEntityInEnergyProductionAchievementPrototypeExcluded {
    EntityID(crate::types::EntityID),
    VecEntityID(Vec<crate::types::EntityID>),
}
pub enum DontUseEntityInEnergyProductionAchievementPrototypeIncluded {
    EntityID(crate::types::EntityID),
    VecEntityID(Vec<crate::types::EntityID>),
}
