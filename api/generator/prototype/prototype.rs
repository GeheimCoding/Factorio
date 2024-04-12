#![allow(unused)]
use std::{collections::HashSet, fmt::Display, str::FromStr};

use serde::{Deserialize, Deserializer};

use crate::generator::{generate_docs, generate_struct, Generate};

use super::{
    image::Image,
    property::{CustomProperties, Property},
};

#[derive(Debug, Deserialize)]
pub struct Prototype {
    /// The name of the prototype.
    pub name: String,
    /// The order of the prototype as shown in the HTML.
    order: u16,
    /// The text description of the prototype.
    description: String,
    /// A list of Markdown lists to provide additional information. Usually contained in a spoiler tag.
    lists: Option<Vec<String>>,
    /// A list of code-only examples about the prototype.
    examples: Option<Vec<String>>,
    /// A list of illustrative images shown next to the prototype.
    images: Option<Vec<Image>>,
    /// The name of the prototype's parent, if any.
    parent: Option<String>,
    /// Whether the prototype is abstract, and thus can't be created directly.
    #[serde(rename = "abstract")]
    pub abstract_: bool,
    /// The type name of the prototype, like `"boiler"`. `null` for abstract prototypes.
    typename: Option<String>,
    /// instance_limit :: number (optional): The maximum number of instances of this prototype that can be created, if any.
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_number_from_string")]
    instance_limit: Option<u8>,
    /// Whether the prototype is deprecated and shouldn't be used anymore.
    deprecated: bool,
    /// The list of properties that the prototype has. May be an empty array.
    properties: Vec<Property>,
    /// A special set of properties that the user can add an arbitrary number of. Specifies the type of the key and value of the custom property.
    custom_properties: Option<CustomProperties>,
}

impl Generate for Prototype {
    fn generate(
        &self,
        _prefix: String,
        _enum_variant: bool,
        indent: usize,
        _unions: &mut Vec<String>,
        class_names: &HashSet<String>,
    ) -> String {
        // TODO: typename & custom_properties?
        // TODO: fix doc string, see CraftingMachinePrototype
        format!(
            "{}{}",
            generate_docs(
                Some(&self.description),
                self.lists.as_ref(),
                None,
                self.examples.as_ref(),
                indent,
            ),
            generate_struct(
                &self.name,
                self.parent.as_ref(),
                &self.properties,
                class_names
            )
        )
    }
}

fn deserialize_optional_number_from_string<'de, D, T>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    String::deserialize(deserializer)?
        .parse::<T>()
        .map(|n| Some(n))
        .map_err(serde::de::Error::custom)
}
