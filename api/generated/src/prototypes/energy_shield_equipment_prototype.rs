#[derive(Debug, serde::Deserialize)]
pub struct EnergyShieldEquipmentPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EquipmentPrototype,
    energy_per_shield: crate::types::Energy,
    max_shield_value: f32,
}
