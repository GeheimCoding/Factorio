#[derive(Debug, serde::Deserialize)]
pub struct UseEntityInEnergyProductionAchievementPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::AchievementPrototype,
    consumed_condition: Option<crate::types::ItemIDFilter>,
    entity: crate::types::EntityID,
    produced_condition: Option<crate::types::ItemIDFilter>,
    required_to_build: Option<crate::types::EntityID>,
}
