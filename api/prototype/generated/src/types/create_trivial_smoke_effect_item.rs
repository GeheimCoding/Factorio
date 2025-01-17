#[derive(serde::Deserialize)]
pub struct CreateTrivialSmokeEffectItem {
    base_: crate::types::TriggerEffectItem,
    initial_height: f32,
    max_radius: f32,
    offset_deviation: crate::types::BoundingBox,
    offsets: Vec<crate::types::Vector>,
    smoke_name: crate::types::TrivialSmokeID,
    speed: crate::types::Vector,
    speed_from_center: f32,
    speed_from_center_deviation: f32,
    speed_multiplier: f32,
    speed_multiplier_deviation: f32,
    starting_frame: f32,
    starting_frame_deviation: f32,
    type_: String,
}
