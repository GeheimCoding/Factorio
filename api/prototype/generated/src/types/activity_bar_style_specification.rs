#[derive(serde::Deserialize)]
pub struct ActivityBarStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    bar: crate::types::ElementImageSet,
    bar_background: crate::types::ElementImageSet,
    bar_size_ratio: f32,
    bar_width: u32,
    color: crate::types::Color,
    speed: f32,
    type_: String,
}
