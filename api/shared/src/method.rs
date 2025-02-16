use crate::basic_member::BasicMember;
use crate::event_raised::EventRaised;
use crate::method_format::MethodFormat;
use crate::parameter::Parameter;
use crate::parameter_group::ParameterGroup;
use crate::variadic_parameter::VariadicParameter;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Method {
    #[serde(flatten)]
    pub base: BasicMember,
    pub visibility: Option<Vec<String>>,
    pub raises: Option<Vec<EventRaised>>,
    pub subclasses: Option<Vec<String>>,
    pub parameters: Vec<Parameter>,
    pub variant_parameter_groups: Option<Vec<ParameterGroup>>,
    pub variant_parameter_description: Option<String>,
    pub variadic_parameter: Option<VariadicParameter>,
    pub format: MethodFormat,
    pub return_values: Vec<Parameter>,
}
