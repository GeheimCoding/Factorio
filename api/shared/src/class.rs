use crate::attribute::Attribute;
use crate::basic_member::BasicMember;
use crate::method::Method;
use crate::operator::Operator;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Class {
    #[serde(flatten)]
    pub base: BasicMember,
    pub visibility: Option<Vec<String>>,
    pub parent: Option<String>,
    #[serde(rename = "abstract")]
    pub abstract_: bool,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
    pub operators: Vec<Operator>,
}
