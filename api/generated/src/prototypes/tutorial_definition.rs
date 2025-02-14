#[derive(Debug, serde::Deserialize)]
pub struct TutorialDefinition {
    #[serde(flatten)]
    base_: crate::prototypes::PrototypeBase,
    // default: Value of `name`
    order: Option<crate::types::Order>,
    scenario: String,
}
