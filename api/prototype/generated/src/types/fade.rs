#[derive(Debug, serde::Deserialize)]
pub enum Fade {
    #[serde(untagged)]
    Fade {
        base_: crate::types::Attenuation,
        from: Option<crate::types::ControlPoint>,
        to: Option<crate::types::ControlPoint>,
    },
    #[serde(untagged)]
    AttenuationType(crate::types::AttenuationType),
}
