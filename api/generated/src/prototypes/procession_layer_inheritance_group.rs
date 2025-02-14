#[derive(Debug, serde::Deserialize)]
pub struct ProcessionLayerInheritanceGroup {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
    arrival_application: Option<crate::types::TransitionApplication>,
    intermezzo_application: Option<crate::types::TransitionApplication>,
}
