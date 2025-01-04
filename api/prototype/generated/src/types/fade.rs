pub enum Fade {
    Fade {
        from: ControlPoint,
        to: ControlPoint,
    },
    AttenuationType(AttenuationType),
}
