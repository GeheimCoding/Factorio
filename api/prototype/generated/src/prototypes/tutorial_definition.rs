#[derive(serde::Deserialize)]
pub struct TutorialDefinition {
    base_: crate::prototypes::PrototypeBase,
    order: crate::types::Order,
    scenario: String,
}
