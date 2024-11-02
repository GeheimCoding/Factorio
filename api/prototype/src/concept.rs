use crate::basic_member::BasicMember;
use crate::property::Property;
use crate::type_::Type;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Concept {
    #[serde(flatten)]
    base: BasicMember,
    parent: Option<String>,
    #[serde(rename = "abstract")]
    abstract_: bool,
    inline: bool,
    #[serde(rename = "type")]
    type_: Type,
    properties: Option<Vec<Property>>,
}
