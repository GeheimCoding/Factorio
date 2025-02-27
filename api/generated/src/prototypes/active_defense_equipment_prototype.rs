#[derive(Debug, serde::Deserialize)]
pub struct ActiveDefenseEquipmentPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EquipmentPrototype,
    attack_parameters: crate::types::AttackParameters,
    automatic: bool,
}
