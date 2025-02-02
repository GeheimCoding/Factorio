#[derive(Debug, serde::Deserialize)]
pub struct BatteryEquipmentPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EquipmentPrototype,
}
