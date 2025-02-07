#[derive(Debug, serde::Deserialize)]
pub struct SpidertronRemotePrototype {
    #[serde(flatten)]
    base_: crate::prototypes::SelectionToolPrototype,
    stack_size: f64,
}
