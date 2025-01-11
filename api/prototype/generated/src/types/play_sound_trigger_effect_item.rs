pub struct PlaySoundTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    audible_distance_modifier: f32,
    max_distance: f32,
    min_distance: f32,
    play_on_target_position: bool,
    sound: crate::types::Sound,
    type_: String,
    volume_modifier: f32,
}
