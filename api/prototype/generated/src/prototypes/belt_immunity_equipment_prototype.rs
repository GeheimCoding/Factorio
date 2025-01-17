#[derive(serde::Deserialize)]
pub struct BeltImmunityEquipmentPrototype {
    base_: crate::prototypes::EquipmentPrototype,
    energy_consumption: crate::types::Energy,
}
