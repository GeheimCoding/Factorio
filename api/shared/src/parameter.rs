use crate::type_::Type;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub order: u16,
    pub description: String,
    #[serde(rename = "type")]
    pub type_: Type,
    pub optional: bool,
}
