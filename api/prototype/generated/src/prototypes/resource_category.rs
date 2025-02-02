#[derive(Debug, serde::Deserialize)]
pub struct ResourceCategory {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
}
