#[derive(Debug, serde::Deserialize)]
pub struct SpidertronRemotePrototype {
    base_: crate::prototypes::SelectionToolPrototype,
    stack_size: f64,
}
