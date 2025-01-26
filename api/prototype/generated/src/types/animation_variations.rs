#[derive(Debug, serde::Deserialize)]
pub enum AnimationVariations {
    #[serde(untagged)]
    AnimationVariations {
        sheet: Option<crate::types::AnimationSheet>,
        sheets: Option<Vec<crate::types::AnimationSheet>>,
    },
    #[serde(untagged)]
    Animation(Box<crate::types::Animation>),
    #[serde(untagged)]
    VecAnimation(Vec<crate::types::Animation>),
}
