#[derive(serde::Deserialize)]
pub struct CameraEffectTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    delay: u8,
    duration: u8,
    ease_in_duration: u8,
    ease_out_duration: u8,
    full_strength_max_distance: u16,
    max_distance: u16,
    strength: f32,
    #[serde(rename = "type")]
    type_: String,
}
