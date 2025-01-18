#[derive(serde::Deserialize)]
pub struct FontPrototype {
    border: bool,
    border_color: crate::types::Color,
    filtered: bool,
    from: String,
    name: String,
    size: i32,
    spacing: f32,
    #[serde(rename = "type")]
    type_: String,
}
