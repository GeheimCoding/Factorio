#[derive(Debug, serde::Deserialize)]
pub struct ColumnWidth {
    base_: crate::types::ColumnWidthItem,
    column: u32,
}
