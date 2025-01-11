pub struct SegmentPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    animation: crate::types::RotatedAnimation,
    backward_overlap: u8,
    backward_padding: f64,
    dying_sound: crate::types::Sound,
    dying_sound_volume_modifier: f32,
    forward_overlap: u8,
    forward_padding: f64,
    render_layer: crate::types::RenderLayer,
    update_effects: Vec<crate::types::TriggerEffectWithCooldown>,
    update_effects_while_enraged: Vec<crate::types::TriggerEffectWithCooldown>,
}
