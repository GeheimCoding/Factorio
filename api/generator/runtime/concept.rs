#![allow(dead_code)]
use serde::Deserialize;

use crate::generator::prototype::type_::Type;

#[derive(Debug, Deserialize)]
pub struct Concept {
    /// The name of the concept.
    name: String,
    /// The order of the concept as shown in the HTML.
    order: u16,
    /// The text description of the concept.
    description: String,
    /// A list of strings containing additional information about the concept.
    notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    examples: Option<Vec<String>>,
    /// The type of the concept.
    #[serde(rename = "type")]
    type_: Type,
}
