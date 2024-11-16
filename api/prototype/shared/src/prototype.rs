use crate::basic_member::BasicMember;
use crate::custom_property::CustomProperty;
use crate::property::Property;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Prototype {
    #[serde(flatten)]
    pub base: BasicMember,
    pub visibility: Option<Vec<String>>,
    pub parent: Option<String>,
    #[serde(rename = "abstract")]
    pub abstract_: bool,
    pub typename: Option<String>,
    pub instance_limit: Option<u64>,
    pub deprecated: bool,
    pub properties: Vec<Property>,
    pub custom_properties: Option<CustomProperty>,
}
