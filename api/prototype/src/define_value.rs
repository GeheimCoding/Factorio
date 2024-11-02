use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DefineValue {
    name: String,
    order: u16,
    description: String,
}
