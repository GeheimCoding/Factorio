#[derive(serde::Deserialize)]
pub struct ColorHintSpecification {
    text: String,
    text_color: crate::types::Color,
}
