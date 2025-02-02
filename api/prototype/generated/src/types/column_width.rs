#[derive(Debug, serde::Deserialize)]
pub struct ColumnWidth {
    #[serde(flatten)]
    base_: crate::types::ColumnWidthItem,
    column: u32,
}
