#[derive(Debug, serde::Deserialize)]
pub struct ItemWithTagsPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::ItemWithLabelPrototype,
}
