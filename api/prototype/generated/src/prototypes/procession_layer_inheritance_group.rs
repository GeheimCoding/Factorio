#[derive(serde::Deserialize)]
pub struct ProcessionLayerInheritanceGroup {
    base_: crate::prototypes::Prototype,
    arrival_application: crate::types::TransitionApplication,
    intermezzo_application: crate::types::TransitionApplication,
}
