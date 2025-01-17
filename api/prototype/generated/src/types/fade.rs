#[derive(serde::Deserialize)]
pub enum Fade {
    #[serde(untagged)]
    Fade {
        base_: crate::types::Attenuation,
        from: crate::types::ControlPoint,
        to: crate::types::ControlPoint,
    },
    #[serde(untagged)]
    AttenuationType(crate::types::AttenuationType),
}
