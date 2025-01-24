#[derive(serde::Deserialize)]
pub struct CreateFireTriggerEffectItem {
    base_: crate::types::CreateEntityTriggerEffectItem,
    // default: MAX_UINT8
    initial_ground_flame_count: Option<u8>,
    #[serde(rename = "type")]
    type_: String,
}
