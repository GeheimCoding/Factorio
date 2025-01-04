pub enum ControlPoint {
    ControlPoint {
        control: f32,
        volume_percentage: f32,
    },
    F32F32((f32, f32)),
}
