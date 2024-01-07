#![allow(unused)]
use std::{fs, io};

use serde::Deserialize;

use crate::generator::{generate, Import};

use super::{concept::Concept, prototype::Prototype};

/// ## [Prototype JSON Format](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-prototype.html#prototype-json-format)
///
/// The prototype API documentation is available in a machine-readable [JSON format](https://lua-api.factorio.com/1.1.101/prototype-api.json). It allows for the creation of developer tools that provide code completion and related functionality. This page documents the structure of this format.
///
/// The current `api_version` that these docs reflect is `4`, which was introduced with Factorio `1.1.89`. See [Changelog](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-prototype.html#Changelog).
///
/// ## [General notes](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-prototype.html#general-notes)
///
/// Some notes that apply to the format in general:
/// - If a member would be `null`, it is omitted from the JSON instead.
/// - Descriptions are generally empty (`""`) instead of `null` if they could exist on any given member, but just happen to be empty (ex. an empty attribute description).
/// - Every list is sorted alphabetically by name. To replicate the order seen on the website, it can be sorted by the `order` property of its members.
/// - Text (descriptions, examples, etc.) is formatted as [Markdown](https://daringfireball.net/projects/markdown/), which includes links, inline code, and code blocks. More on how links work right below.
///
/// ## [Link format](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-prototype.html#link-format)
///
/// All text can contain Markdown-type links. There are two broad categories for these: hyperlinks that link to any external website, and internal links that refer to another part of this documentation. All of them will have a title that should be displayed as the link's text.
///
/// - External links work like standard Markdown links, meaning they always start with `https://`, and are followed by the URL. Example: `[Factorio](https://factorio.com)`.
/// - Internal links are a bit more complex. They aren't valid hyperlinks, but instead use a custom shorthand format to refer to specific parts of the API. This format has three main parts:
///     - They always start with either `runtime:` or `prototype:`, indicating the stage that is linked to. The two stages are separate namespaces, as there would be naming conflicts otherwise. So this first part indicates whether to look for the API member among classes, events, etc., or among prototypes and types.
///     - The second part is the name of the API member being linked to. What this can be depends on the stage that's indicated beforehand. Examples would be `LuaGuiElement` or `on_player_created` for `runtime:`, and `RecipePrototype` or `EnergySource` for `prototype:`.
///         - Note that this can be the name of some stage-specific auxiliary pages instead. Namely, 'classes', 'events', 'concepts', 'defines' and 'builtin_types' for `runtime:`, and 'prototypes', 'types' for `prototype:`.
///     - The third, optional part of an internal link can specify a certain sub-member to refer to. Its format is `::<name>`, where name is the name of a class method or attribute, or a prototype/type property. It is invalid for any other member type.
///
/// Examples:
///
/// - `[LuaGuiElement](runtime:LuaGuiElement)` links to the `LuaGuiElement` class.
/// - `[results](prototype:RecipePrototype::results)` links to the `results` property of the `RecipePrototype` prototype.
/// - `[concepts](runtime:concepts)` links to the [Concepts](https://lua-api.factorio.com/1.1.101/concepts.html) overview page.
#[derive(Debug, Deserialize)]
pub struct PrototypeApiFormat {
    /// The application this documentation is for. Will always be `"factorio"`.
    application: String,
    /// Indicates the stage this documentation is for. Will always be `"prototype"` (as opposed to `"runtime"`; see the [data lifecycle](https://lua-api.factorio.com/1.1.101/auxiliary/data-lifecycle.html) for more detail).
    stage: String,
    /// The version of the game that this documentation is for. An example would be `"1.1.90"`.
    application_version: String,
    /// The version of the machine-readable format itself. It is incremented every time the format changes. The version this documentation reflects is stated at the top.
    api_version: u8,
    /// The list of prototypes that can be created. Equivalent to the [prototypes](https://lua-api.factorio.com/1.1.101/prototypes.html) page.
    prototypes: Vec<Prototype>,
    /// The list of types (concepts) that the format uses. Equivalent to the [types](https://lua-api.factorio.com/1.1.101/types.html) page.
    types: Vec<Concept>,
}

impl PrototypeApiFormat {
    pub fn generate_prototype_api(
        &self,
        prototypes_path: &str,
        types_path: &str,
    ) -> io::Result<()> {
        fs::write(
            prototypes_path,
            generate(&self.prototypes, vec![Import::Types]),
        )?;
        fs::write(types_path, generate(&self.types, vec![Import::HashMap]))
    }

    pub fn generate_factorio_types(&self) -> String {
        let mut result = String::from("pub enum Prototype {\n");
        for proto in &self.prototypes {
            result.push_str(&format!(
                "    {}(super::prototypes::{}),\n",
                proto.name, proto.name
            ));
        }
        result.push_str("}\n\npub enum Type {\n");
        for concept in &self.types {
            if concept.name.chars().next().unwrap().is_uppercase() {
                result.push_str(&format!(
                    "    {}(super::types::{}),\n",
                    concept.name, concept.name
                ));
            }
        }
        result.push_str("}\n");
        result
    }
}
