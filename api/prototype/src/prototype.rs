use crate::basic_member::BasicMember;
use crate::custom_property::CustomProperty;
use crate::property::Property;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Prototype {
    #[serde(flatten)]
    base: BasicMember,
    visibility: Option<Vec<String>>,
    parent: Option<String>,
    #[serde(rename = "abstract")]
    abstract_: bool,
    typename: Option<String>,
    instance_limit: Option<u64>,
    deprecated: bool,
    properties: Vec<Property>,
    custom_properties: Option<CustomProperty>,
}
