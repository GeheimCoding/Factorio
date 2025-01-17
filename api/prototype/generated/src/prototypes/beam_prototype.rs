#[derive(serde::Deserialize)]
pub struct BeamPrototype {
    base_: crate::prototypes::EntityPrototype,
    action: crate::types::Trigger,
    action_triggered_automatically: bool,
    damage_interval: u32,
    graphics_set: crate::types::BeamGraphicsSet,
    random_target_offset: bool,
    target_offset: crate::types::Vector,
    width: f32,
}
