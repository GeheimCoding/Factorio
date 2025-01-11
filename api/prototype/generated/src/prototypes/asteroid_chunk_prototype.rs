pub struct AsteroidChunkPrototype {
    base_: crate::prototypes::Prototype,
    collision_box: crate::types::SimpleBoundingBox,
    dying_trigger_effect: crate::types::TriggerEffect,
    graphics_set: crate::types::AsteroidGraphicsSet,
    hide_from_signal_gui: bool,
    icon: crate::types::FileName,
    icon_size: crate::types::SpriteSizeType,
    icons: Vec<crate::types::IconData>,
    minable: crate::types::MinableProperties,
}
