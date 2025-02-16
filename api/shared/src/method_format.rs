use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MethodFormat {
    pub takes_table: bool,
    pub table_optional: Option<bool>,
}
