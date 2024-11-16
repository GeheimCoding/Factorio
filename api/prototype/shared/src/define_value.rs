use crate::pascal_case::PascalCase;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DefineValue {
    pub name: String,
    pub order: u16,
    pub description: String,
}

impl DefineValue {
    pub fn generate(&self) -> String {
        format!("{},", self.name.to_pascal_case())
    }
}
