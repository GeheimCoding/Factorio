#![allow(unused)]
use serde::Deserialize;

use crate::generator::{generate_docs, type_::Type, Generate, StringTransformation};

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

impl Generate for Attribute {
    fn generate(
        &self,
        prefix: String,
        enum_variant: bool,
        indent: usize,
        unions: &mut Vec<String>,
    ) -> String {
        let prefix = format!("{prefix}{}", self.name.to_pascal_case());
        let mut result = generate_docs(Some(&self.description), None, None, None, indent);
        result.push_str(&format!(
            "    {}: {},",
            self.name.to_rust_field_name(),
            self.type_
                .generate(prefix, enum_variant, indent, unions)
                .to_optional_if(self.optional)
        ));
        result
    }
}
