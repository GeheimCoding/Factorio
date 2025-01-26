#[derive(Debug, serde::Deserialize)]
pub struct DoubleSliderStyleSpecification {
    base_: crate::types::SliderStyleSpecification,
    #[serde(rename = "type")]
    type_: String,
}
