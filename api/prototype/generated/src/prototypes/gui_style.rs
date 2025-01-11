pub struct GuiStyle {
    base_: crate::prototypes::PrototypeBase,
    default_sprite_priority: crate::types::SpritePriority,
    default_sprite_scale: f64,
    default_tileset: crate::types::FileName,
    custom_: std::collections::HashMap<String, crate::types::StyleSpecification>,
}
