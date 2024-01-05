#![allow(dead_code)]
use serde::Deserialize;

use crate::generator::prototype::type_::Type;

use super::{
    event::EventRaised,
    parameter::{Parameter, ParameterGroup},
};

#[derive(Debug, Deserialize)]
pub struct Method {
    /// The name of the method.
    name: String,
    /// The order of the method as shown in the HTML.
    order: u16,
    /// The text description of the method.
    description: String,
    /// A list of strings containing additional information about the method.
    notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    examples: Option<Vec<String>>,
    /// A list of events that this method might raise when called.
    raises: Option<Vec<EventRaised>>,
    /// A list of strings specifying the sub-type (of the class) that the method applies to.
    subclasses: Option<Vec<String>>,
    /// The parameters of the method. How to interpret them depends on the `takes_table` member.
    parameters: Vec<Parameter>,
    /// The optional parameters that depend on one of the main parameters. Only applies if `takes_table` is `true`.
    variant_parameter_groups: Option<Vec<ParameterGroup>>,
    /// The text description of the optional parameter groups.
    variant_parameter_description: Option<String>,
    /// The type of the variadic arguments of the method, if it accepts any.
    variadic_type: Option<Type>,
    /// The description of the variadic arguments of the method, if it accepts any.
    variadic_description: Option<String>,
    /// Whether the method takes a single table with named parameters or a sequence of unnamed parameters.
    takes_table: bool,
    /// If `takes_table` is `true`, whether that whole table is optional or not.
    table_is_optional: Option<bool>,
    /// The return values of this method, which can contain zero, one, or multiple values. Note that these have the same structure as parameters, but do not specify a name.
    return_values: Vec<Parameter>,
}
