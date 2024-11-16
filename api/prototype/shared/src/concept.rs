use crate::basic_member::BasicMember;
use crate::property::Property;
use crate::type_::Type;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Concept {
    #[serde(flatten)]
    pub base: BasicMember,
    pub parent: Option<String>,
    #[serde(rename = "abstract")]
    pub abstract_: bool,
    pub inline: bool,
    #[serde(rename = "type")]
    pub type_: Type,
    pub properties: Option<Vec<Property>>,
}
