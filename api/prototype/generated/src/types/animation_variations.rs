#[derive(serde::Deserialize)]
pub enum AnimationVariations {
    #[serde(untagged)]
    AnimationVariations {
        sheet: crate::types::AnimationSheet,
        sheets: Vec<crate::types::AnimationSheet>,
    },
    #[serde(untagged)]
    Animation(Box<crate::types::Animation>),
    #[serde(untagged)]
    VecAnimation(Vec<crate::types::Animation>),
}
