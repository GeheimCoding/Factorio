pub enum AnimationVariations {
    AnimationVariations {
        sheet: crate::types::AnimationSheet,
        sheets: Vec<crate::types::AnimationSheet>,
    },
    Animation(Box<crate::types::Animation>),
    VecAnimation(Vec<crate::types::Animation>),
}
