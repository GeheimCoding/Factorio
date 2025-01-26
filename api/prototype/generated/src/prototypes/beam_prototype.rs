#[derive(Debug, serde::Deserialize)]
pub struct BeamPrototype {
    base_: crate::prototypes::EntityPrototype,
    action: Option<crate::types::Trigger>,
    #[serde(default = "default_action_triggered_automatically")]
    action_triggered_automatically: bool,
    damage_interval: u32,
    graphics_set: crate::types::BeamGraphicsSet,
    #[serde(default = "default_random_target_offset")]
    random_target_offset: bool,
    target_offset: Option<crate::types::Vector>,
    width: f32,
}
fn default_action_triggered_automatically() -> bool {
    false
}
fn default_random_target_offset() -> bool {
    false
}
