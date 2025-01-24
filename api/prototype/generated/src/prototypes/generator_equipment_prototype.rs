#[derive(serde::Deserialize)]
pub struct GeneratorEquipmentPrototype {
    base_: crate::prototypes::EquipmentPrototype,
    burner: Option<crate::types::BurnerEnergySource>,
    power: crate::types::Energy,
}
