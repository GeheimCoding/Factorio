#[derive(Debug, serde::Deserialize)]
pub struct ColumnWidthItem {
    #[serde(default = "default_maximal_width")]
    maximal_width: i32,
    #[serde(default = "default_minimal_width")]
    minimal_width: i32,
    #[serde(default = "default_width")]
    width: i32,
}
fn default_maximal_width() -> i32 {
    0
}
fn default_minimal_width() -> i32 {
    0
}
fn default_width() -> i32 {
    0
}
