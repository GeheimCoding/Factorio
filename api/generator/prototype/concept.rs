#![allow(unused)]
use std::collections::HashSet;

use serde::Deserialize;

use crate::generator::{
    generate_docs, generate_struct, generate_union,
    type_::{ComplexType, Type},
    Generate, Macro,
};

use super::{image::Image, property::Property};

#[derive(Debug, Deserialize)]
pub struct Concept {
    /// The name of the type.
    pub name: String,
    /// The order of the type as shown in the HTML.
    order: u16,
    /// The text description of the type.
    description: String,
    /// A list of Markdown lists to provide additional information. Usually contained in a spoiler tag.
    lists: Option<Vec<String>>,
    /// A list of code-only examples about the type.
    examples: Option<Vec<String>>,
    /// A list of illustrative images shown next to the type.
    images: Option<Vec<Image>>,
    /// The name of the type's parent, if any.
    parent: Option<String>,
    /// Whether the type is abstract, and thus can't be created directly.
    #[serde(rename = "abstract")]
    pub abstract_: bool,
    /// Whether the type is inlined inside another property's description.
    inline: bool,
    /// The type of the type/concept (Yes, this naming is confusing). Either a proper [Type](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-prototype.html#Type), or the string `"builtin"`, indicating a fundamental type like `string` or `number`.
    #[serde(rename = "type")]
    type_: Type,
    /// The list of properties that the type has, if its type includes a struct. `null` otherwise.
    properties: Option<Vec<Property>>,
}

impl Generate for Concept {
    fn generate(
        &self,
        _prefix: String,
        _enum_variant: bool,
        indent: usize,
        unions: &mut Vec<String>,
        class_names: &HashSet<String>,
    ) -> String {
        let mut result = String::from(generate_docs(
            Some(&self.description),
            self.lists.as_ref(),
            None,
            self.examples.as_ref(),
            indent,
        ));
        let is_new_type = match &self.type_ {
            Type::Simple(string) => {
                if string == "builtin" {
                    return String::new();
                }
                true
            }
            Type::Complex(complex) => match complex.as_ref() {
                ComplexType::Struct => {
                    result.push_str(&generate_struct(
                        &self.name,
                        self.parent.as_ref(),
                        self.properties
                            .as_ref()
                            .expect("there should be at least one property"),
                        class_names,
                    ));
                    false
                }
                ComplexType::Union {
                    options,
                    full_format: _,
                } => {
                    generate_union(
                        &self.name,
                        options,
                        unions,
                        self.properties.as_ref(),
                        class_names,
                    );
                    false
                }
                _ => true,
            },
        };
        if is_new_type {
            let mut new_type = format!(
                "pub type {} = {};",
                self.name,
                self.type_.generate(
                    format!("{}Union", self.name),
                    false,
                    indent,
                    unions,
                    class_names
                )
            );
            result.push_str(&new_type);
            if self.name == "BoundingBox" {
                result.push_str(
                    &new_type
                        .replace("MapPosition, MapPosition", "MapPosition, MapPosition, f32")
                        .replace("BoundingBox", "BoundingBoxExtra"),
                );
            }
            if !unions.is_empty() {
                result.push_str("\n\n");
            }
        }
        result.push_str(&unions.join("\n\n"));

        result
    }
}
