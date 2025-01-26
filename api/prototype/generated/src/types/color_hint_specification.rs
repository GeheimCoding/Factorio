#[derive(Debug, serde::Deserialize)]
pub struct ColorHintSpecification {
    #[serde(default = "default_text")]
    text: String,
    text_color: Option<crate::types::Color>,
}
fn default_text() -> String {
    String::from("")
}
