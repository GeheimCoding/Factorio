use crate::attribute::Attribute;
use crate::method::Method;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Operator {
    Method(Method),
    Attribute(Attribute),
}
