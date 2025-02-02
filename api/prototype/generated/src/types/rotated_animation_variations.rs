#[derive(Debug, serde::Deserialize)]
pub enum RotatedAnimationVariations {
    #[serde(untagged)]
    RotatedAnimation(Box<crate::types::RotatedAnimation>),
    #[serde(untagged)]
    VecRotatedAnimation(crate::vec::Vec<crate::types::RotatedAnimation>),
}
