#[derive(Debug, serde::Deserialize)]
pub struct ActivityBarStyleSpecification {
    base_: crate::types::BaseStyleSpecification,
    bar: Option<crate::types::ElementImageSet>,
    bar_background: Option<crate::types::ElementImageSet>,
    bar_size_ratio: Option<f32>,
    bar_width: Option<u32>,
    color: Option<crate::types::Color>,
    speed: Option<f32>,
}
