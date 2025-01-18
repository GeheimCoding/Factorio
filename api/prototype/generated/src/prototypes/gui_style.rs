#[derive(serde::Deserialize)]
pub struct GuiStyle {
    base_: crate::prototypes::PrototypeBase,
    #[serde(default = "default_default_sprite_priority")]
    default_sprite_priority: crate::types::SpritePriority,
    #[serde(default = "default_default_sprite_scale")]
    default_sprite_scale: f64,
    #[serde(default = "default_default_tileset")]
    default_tileset: crate::types::FileName,
    custom_: std::collections::HashMap<String, crate::types::StyleSpecification>,
}
fn default_default_sprite_priority() -> crate::types::SpritePriority {
    crate::types::SpritePriority::Medium
}
fn default_default_sprite_scale() -> f64 {
    1.0
}
fn default_default_tileset() -> crate::types::FileName {
    String::from("")
}
