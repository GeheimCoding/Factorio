#[derive(Debug, serde::Deserialize)]
pub struct CreateFireTriggerEffectItem {
    #[serde(flatten)]
    base_: crate::types::CreateEntityTriggerEffectItem,
    // default: MAX_UINT8
    initial_ground_flame_count: Option<u8>,
}
