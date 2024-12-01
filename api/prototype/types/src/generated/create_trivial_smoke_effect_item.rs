pub struct CreateTrivialSmokeEffectItem {
    initial_height: f32,
    max_radius: f32,
    offset_deviation: BoundingBox,
    offsets: Vec<Vector>,
    smoke_name: TrivialSmokeID,
    speed: Vector,
    speed_from_center: f32,
    speed_from_center_deviation: f32,
    speed_multiplier: f32,
    speed_multiplier_deviation: f32,
    starting_frame: f32,
    starting_frame_deviation: f32,
    type_: CreateTrivialSmoke,
}
