pub enum Fade {
    Fade {
        base_: crate::types::Attenuation,
        from: crate::types::ControlPoint,
        to: crate::types::ControlPoint,
    },
    AttenuationType(crate::types::AttenuationType),
}
