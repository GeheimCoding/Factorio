#[derive(Debug, serde::Deserialize)]
pub enum AnimationVariations {
    #[serde(untagged)]
    AnimationVariations {
        sheet: Option<Box<crate::types::AnimationSheet>>,
        sheets: Option<crate::vec::Vec<crate::types::AnimationSheet>>,
    },
    #[serde(untagged)]
    Animation(Box<crate::types::Animation>),
    #[serde(untagged)]
    VecAnimation(crate::vec::Vec<crate::types::Animation>),
}
