#[derive(Debug, serde::Deserialize)]
pub struct AsteroidChunkPrototype {
    base_: crate::prototypes::Prototype,
    collision_box: Option<crate::types::SimpleBoundingBox>,
    dying_trigger_effect: Option<crate::types::TriggerEffect>,
    graphics_set: Option<crate::types::AsteroidGraphicsSet>,
    // default: unset
    hide_from_signal_gui: Option<bool>,
    icon: Option<crate::types::FileName>,
    #[serde(default = "default_icon_size")]
    icon_size: crate::types::SpriteSizeType,
    icons: Option<Vec<crate::types::IconData>>,
    minable: Option<crate::types::MinableProperties>,
}
fn default_icon_size() -> crate::types::SpriteSizeType {
    64
}
