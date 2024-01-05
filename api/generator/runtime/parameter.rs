#![allow(unused)]
use serde::Deserialize;

use crate::generator::type_::Type;

#[derive(Debug, Deserialize)]
pub struct Parameter {
    /// The name of the parameter.
    name: Option<String>,
    /// The order of the parameter as shown in the HTML.
    order: u16,
    /// The text description of the parameter.
    description: String,
    /// The type of the parameter.
    #[serde(rename = "type")]
    type_: Type,
    /// Whether the type is optional or not.
    optional: bool,
}

#[derive(Debug, Deserialize)]
pub struct ParameterGroup {
    /// The name of the parameter group.
    name: String,
    /// The order of the parameter group as shown in the HTML.
    order: u16,
    /// The text description of the parameter group.
    description: Option<String>,
    /// The parameters that the group adds.
    parameters: Vec<Parameter>,
}
