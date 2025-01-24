#[derive(serde::Deserialize)]
pub struct ProcessionLayerInheritanceGroup {
    base_: crate::prototypes::Prototype,
    arrival_application: Option<crate::types::TransitionApplication>,
    intermezzo_application: Option<crate::types::TransitionApplication>,
}
