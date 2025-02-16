use crate::parameter::Parameter;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct ParameterGroup {
    pub name: String,
    pub order: u16,
    pub description: String,
    pub parameters: Vec<Parameter>,
}
