#[derive(Debug, serde::Deserialize)]
pub struct AnimatedVector {
    direction_shift: Option<DirectionShift>,
    render_layer: Option<crate::types::RenderLayer>,
    rotations: Vec<VectorRotation>,
}
#[derive(Debug, serde::Deserialize)]
pub struct DirectionShift {
    east: Option<crate::types::Vector>,
    north: Option<crate::types::Vector>,
    south: Option<crate::types::Vector>,
    west: Option<crate::types::Vector>,
}
#[derive(Debug, serde::Deserialize)]
pub struct VectorRotation {
    frames: Vec<crate::types::Vector>,
    render_layer: Option<crate::types::RenderLayer>,
}
