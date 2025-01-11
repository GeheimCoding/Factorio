pub struct LightningPrototype {
    attracted_volume_modifier: f32,
    damage: f64,
    effect_duration: u16,
    energy: crate::types::Energy,
    graphics_set: crate::types::LightningGraphicsSet,
    sound: crate::types::Sound,
    source_offset: crate::types::Vector,
    source_variance: crate::types::Vector,
    strike_effect: crate::types::Trigger,
    time_to_damage: u16,
}
