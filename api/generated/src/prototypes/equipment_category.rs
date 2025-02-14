#[derive(Debug, serde::Deserialize)]
pub struct EquipmentCategory {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
}
