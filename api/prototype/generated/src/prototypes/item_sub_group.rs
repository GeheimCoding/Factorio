#[derive(Debug, serde::Deserialize)]
pub struct ItemSubGroup {
    base_: crate::prototypes::Prototype,
    group: crate::types::ItemGroupID,
}
