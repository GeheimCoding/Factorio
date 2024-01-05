#![allow(dead_code)]
use serde::Deserialize;

use crate::generator::type_::Type;

use super::event::EventRaised;

#[derive(Debug, Deserialize)]
pub struct Attribute {
    /// The name of the attribute.
    name: String,
    /// The order of the attribute as shown in the HTML.
    order: u16,
    /// The text description of the attribute.
    description: String,
    /// A list of strings containing additional information about the attribute.
    notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    examples: Option<Vec<String>>,
    /// A list of events that this attribute might raise when written to.
    raises: Option<Vec<EventRaised>>,
    /// A list of strings specifying the sub-type (of the class) that the attribute applies to.
    subclasses: Option<Vec<String>>,
    /// The type of the attribute.
    #[serde(rename = "type")]
    type_: Type,
    /// Whether the attribute is optional or not.
    optional: bool,
    /// Whether the attribute can be read from.
    read: bool,
    /// Whether the attribute can be written to.
    write: bool,
}
