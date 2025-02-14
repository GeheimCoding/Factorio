#[derive(Debug, serde::Deserialize)]
pub struct ActiveTriggerPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
}
