#[derive(Debug, serde::Deserialize)]
pub struct DoubleSliderStyleSpecification {
    #[serde(flatten)]
    base_: crate::types::SliderStyleSpecification,
}
