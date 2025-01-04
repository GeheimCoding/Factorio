pub enum Fade {
    Fade {
        from: crate::types::ControlPoint,
        to: crate::types::ControlPoint,
    },
    AttenuationType(crate::types::AttenuationType),
}
