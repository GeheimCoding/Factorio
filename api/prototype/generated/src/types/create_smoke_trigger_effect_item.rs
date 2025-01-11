pub struct CreateSmokeTriggerEffectItem {
    base_: crate::types::CreateEntityTriggerEffectItem,
    initial_height: f32,
    speed: crate::types::Vector,
    speed_from_center: f32,
    speed_from_center_deviation: f32,
    speed_multiplier: f32,
    speed_multiplier_deviation: f32,
    starting_frame: f32,
    starting_frame_deviation: f32,
    type_: String,
}
