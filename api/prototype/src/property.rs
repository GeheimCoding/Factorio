use crate::basic_member::BasicMember;
use crate::type_::{ComplexType, Type};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Property {
    #[serde(flatten)]
    base: BasicMember,
    visibility: Option<Vec<String>>,
    alt_name: Option<String>,
    #[serde(rename = "override")]
    override_: bool,
    #[serde(rename = "type")]
    type_: Type,
    optional: bool,
    default: Option<PropertyDefault>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PropertyDefault {
    String(String),
    Literal(ComplexType),
}
