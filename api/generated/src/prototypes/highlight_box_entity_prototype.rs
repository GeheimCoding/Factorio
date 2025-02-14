#[derive(Debug, serde::Deserialize)]
pub struct HighlightBoxEntityPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityPrototype,
}
