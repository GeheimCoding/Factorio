#[derive(Debug, serde::Deserialize)]
pub struct CustomEventPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
}
