pub enum AnimationVariations {
    AnimationVariations {
        sheet: AnimationSheet,
        sheets: Vec<AnimationSheet>,
    },
    Animation(Box<Animation>),
    VecAnimation(Vec<Animation>),
}
