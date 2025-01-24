#[derive(serde::Deserialize)]
pub struct UseEntityInEnergyProductionAchievementPrototype {
    base_: crate::prototypes::AchievementPrototype,
    consumed_condition: Option<crate::types::ItemID>,
    entity: crate::types::EntityID,
    required_to_build: Option<crate::types::EntityID>,
}
