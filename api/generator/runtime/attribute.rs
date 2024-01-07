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
        let new_prefix = format!("{prefix}{}", self.name.to_pascal_case());
        let mut result = generate_docs(Some(&self.description), None, None, None, indent);
        let type_ = self
            .type_
            .generate(new_prefix, enum_variant, indent, unions);
        let type_ = if type_.starts_with("pub struct") {
            let new_type = type_.split("pub struct ").collect::<Vec<_>>()[1]
                .split_whitespace()
                .next()
                .unwrap()
                .to_owned();
            unions.push(type_);
            new_type
        } else {
            type_
        };
        let type_ = if type_ == prefix
            || type_.starts_with("LuaEntity")
            || type_ == "LuaInventory"
            || type_ == "LuaGui"
            || type_ == "LuaForce"
            || type_ == "LuaEquipmentPrototype"
            || type_ == "LuaBurnerOwner"
        {
            format!("Box<{type_}>")
        } else {
            type_
        };
        result.push_str(&format!(
            "    {}: {},",
            self.name.to_rust_field_name(),
            type_.to_optional_if(self.optional)
        ));
        result
    }
}
