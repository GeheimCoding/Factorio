#![allow(unused)]
use std::collections::HashSet;

use serde::Deserialize;

use crate::generator::{
    generate_docs,
    type_::{self, Type},
    Generate, StringTransformation,
};

use super::image::Image;

#[derive(Debug, Deserialize)]
pub struct Property {
    /// The name of the property.
    name: String,
    /// The order of the property as shown in the HTML.
    order: u16,
    /// The text description of the property.
    description: String,
    /// A list of Markdown lists to provide additional information. Usually contained in a spoiler tag.
    lists: Option<Vec<String>>,
    /// A list of code-only examples about the property.
    examples: Option<Vec<String>>,
    /// A list of illustrative images shown next to the property.
    images: Option<Vec<Image>>,
    /// An alternative name for the property. Either this or name can be used to refer to the property.
    alt_name: Option<String>,
    /// Whether the property overrides a property of the same name in one of its parents.
    #[serde(rename = "override")]
    override_: bool,
    /// The type of the property.
    #[serde(rename = "type")]
    type_: Type,
    /// Whether the property is optional and can be omitted. If so, it falls back to a default value.
    optional: bool,
    /// default :: union[string, Literal] (optional): The default value of the property. Either a textual description or a literal value.
    default: Option<PropertyDefaultUnion>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum PropertyDefaultUnion {
    String(String),
    Literal(Type),
}

#[derive(Debug, Deserialize)]
pub struct CustomProperties {
    /// The text description of the property.
    description: String,
    /// A list of Markdown lists to provide additional information. Usually contained in a spoiler tag.
    lists: Option<Vec<String>>,
    /// A list of code-only examples about the property.
    examples: Option<Vec<String>>,
    /// A list of illustrative images shown next to the property.
    images: Option<Vec<Image>>,
    /// The type of the key of the custom property.
    key_type: Type,
    /// The type of the value of the custom property.
    value_type: Type,
}

impl Generate for Property {
    fn generate(
        &self,
        prefix: String,
        enum_variant: bool,
        indent: usize,
        unions: &mut Vec<String>,
        class_names: &HashSet<String>,
    ) -> String {
        // TODO: alt_name & override & default?
        let type_ = self.type_.generate(
            format!("{prefix}{}", self.name.to_pascal_case()),
            enum_variant,
            indent,
            unions,
            class_names,
        );
        let type_ = if type_ == prefix {
            format!("Box<{type_}>")
        } else {
            type_
        };
        format!(
            "{}{}{}: {},",
            generate_docs(
                Some(&self.description),
                self.lists.as_ref(),
                None,
                self.examples.as_ref(),
                indent
            ),
            "    ".repeat(indent),
            self.name.to_rust_field_name(enum_variant),
            type_.to_optional_if(self.optional)
        )
    }
}
