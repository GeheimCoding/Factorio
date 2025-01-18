#[derive(serde::Deserialize)]
pub struct FontPrototype {
    #[serde(default = "default_border")]
    border: bool,
    border_color: crate::types::Color,
    #[serde(default = "default_filtered")]
    filtered: bool,
    from: String,
    name: String,
    size: i32,
    #[serde(default = "default_spacing")]
    spacing: f32,
    #[serde(rename = "type")]
    type_: String,
}
fn default_border() -> bool {
    false
}
fn default_filtered() -> bool {
    false
}
fn default_spacing() -> f32 {
    0.0
}
