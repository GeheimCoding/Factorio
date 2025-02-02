#[derive(Debug, serde::Deserialize)]
pub struct BeltImmunityEquipmentPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EquipmentPrototype,
    energy_consumption: crate::types::Energy,
}
