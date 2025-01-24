#[derive(serde::Deserialize)]
pub struct TutorialDefinition {
    base_: crate::prototypes::PrototypeBase,
    // default: Value of `name`
    order: Option<crate::types::Order>,
    scenario: String,
}
