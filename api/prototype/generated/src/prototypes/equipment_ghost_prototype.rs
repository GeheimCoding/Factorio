#[derive(Debug, serde::Deserialize)]
pub struct EquipmentGhostPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EquipmentPrototype,
}
