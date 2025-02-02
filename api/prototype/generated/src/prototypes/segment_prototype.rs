#[derive(Debug, serde::Deserialize)]
pub struct SegmentPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    animation: crate::types::RotatedAnimation,
    #[serde(default = "default_backward_overlap")]
    backward_overlap: u8,
    #[serde(default = "default_backward_padding")]
    backward_padding: f64,
    dying_sound: Option<crate::types::Sound>,
    #[serde(default = "default_dying_sound_volume_modifier")]
    dying_sound_volume_modifier: f32,
    #[serde(default = "default_forward_overlap")]
    forward_overlap: u8,
    #[serde(default = "default_forward_padding")]
    forward_padding: f64,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    update_effects: Option<Vec<crate::types::TriggerEffectWithCooldown>>,
    update_effects_while_enraged: Option<Vec<crate::types::TriggerEffectWithCooldown>>,
}
fn default_backward_overlap() -> u8 {
    0
}
fn default_backward_padding() -> f64 {
    0.0
}
fn default_dying_sound_volume_modifier() -> f32 {
    1.0
}
fn default_forward_overlap() -> u8 {
    0
}
fn default_forward_padding() -> f64 {
    0.0
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
