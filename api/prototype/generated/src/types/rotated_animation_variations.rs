#[derive(serde::Deserialize)]
pub enum RotatedAnimationVariations {
    #[serde(untagged)]
    RotatedAnimation(Box<crate::types::RotatedAnimation>),
    #[serde(untagged)]
    VecRotatedAnimation(Vec<crate::types::RotatedAnimation>),
}
