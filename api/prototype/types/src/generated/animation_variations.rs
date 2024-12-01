pub enum AnimationVariations {
    AnimationVariations {
        sheet: AnimationSheet,
        sheets: Vec<AnimationSheet>,
    },
    Animation(Animation),
    VecAnimation(Vec<Animation>),
}
