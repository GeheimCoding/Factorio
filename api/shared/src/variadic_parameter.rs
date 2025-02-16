use crate::type_::Type;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct VariadicParameter {
    #[serde(rename = "type")]
    pub type_: Option<Type>,
    pub description: Option<String>,
}
