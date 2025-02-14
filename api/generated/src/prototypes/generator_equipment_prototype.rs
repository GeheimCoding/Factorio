#[derive(Debug, serde::Deserialize)]
pub struct GeneratorEquipmentPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EquipmentPrototype,
    burner: Option<crate::types::BurnerEnergySource>,
    power: crate::types::Energy,
}
