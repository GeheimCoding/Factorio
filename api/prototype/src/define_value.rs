use crate::pascal_case::PascalCase;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DefineValue {
    name: String,
    order: u16,
    description: String,
}

impl DefineValue {
    pub fn generate(&self) -> String {
        format!("{},", self.name.to_pascal_case())
    }
}
