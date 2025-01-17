#[derive(serde::Deserialize)]
pub struct AnimatedVector {
    direction_shift: DirectionShift,
    render_layer: crate::types::RenderLayer,
    rotations: Vec<VectorRotation>,
}
#[derive(serde::Deserialize)]
pub struct DirectionShift {
    east: crate::types::Vector,
    north: crate::types::Vector,
    south: crate::types::Vector,
    west: crate::types::Vector,
}
#[derive(serde::Deserialize)]
pub struct VectorRotation {
    frames: Vec<crate::types::Vector>,
    render_layer: crate::types::RenderLayer,
}
