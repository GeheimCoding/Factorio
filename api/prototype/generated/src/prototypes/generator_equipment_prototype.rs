#[derive(serde::Deserialize)]
pub struct GeneratorEquipmentPrototype {
    base_: crate::prototypes::EquipmentPrototype,
    burner: crate::types::BurnerEnergySource,
    power: crate::types::Energy,
}
