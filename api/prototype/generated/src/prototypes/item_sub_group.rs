#[derive(Debug, serde::Deserialize)]
pub struct ItemSubGroup {
    #[serde(flatten)]
    base_: crate::prototypes::Prototype,
    group: crate::types::ItemGroupID,
}
