#[derive(Debug, serde::Deserialize)]
pub struct MovementBonusEquipmentPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EquipmentPrototype,
    energy_consumption: crate::types::Energy,
    movement_bonus: f64,
}
