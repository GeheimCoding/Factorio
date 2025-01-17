#[derive(serde::Deserialize)]
pub struct VisualState {
    color: crate::types::Color,
    duration: u8,
    name: String,
    next_active: String,
    next_inactive: String,
}
