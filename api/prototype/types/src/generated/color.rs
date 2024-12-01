pub enum Color {
    Color { a: f32, b: f32, g: f32, r: f32 },
    F32F32F32((f32, f32, f32)),
    F32F32F32F32((f32, f32, f32, f32)),
}
