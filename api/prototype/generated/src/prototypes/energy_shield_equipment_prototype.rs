#[derive(Debug, serde::Deserialize)]
pub struct EnergyShieldEquipmentPrototype {
    base_: crate::prototypes::EquipmentPrototype,
    energy_per_shield: crate::types::Energy,
    max_shield_value: f32,
}
