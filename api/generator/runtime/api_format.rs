#![allow(unused)]
use std::{fs, io};

use serde::Deserialize;

use crate::generator::{generate, StringTransformation};

use super::{
    builtin_type::BuiltinType, class::Class, concept::Concept, define::Define, event::Event,
    global_object::GlobalObject, method::Method,
};

/// ## [Runtime JSON Format](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-runtime.html#runtime-json-format)
///
/// The runtime API documentation is available in a machine-readable [JSON format](https://lua-api.factorio.com/1.1.101/runtime-api.json). It allows for the creation of developer tools that provide code completion and related functionality. This page documents the structure of this format.
///
/// The current api_version that these docs reflect is 4, which was introduced with Factorio 1.1.89. See [Changelog](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-runtime.html#Changelog).
///
/// ## [General notes](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-runtime.html#general-notes)
///
/// Some notes that apply to the format in general:
/// - If a member would be `null`, it is omitted from the JSON instead.
/// - Descriptions are generally empty (`""`) instead of `null` if they could exist on any given member, but just happen to be empty (ex. an empty attribute description).
/// - Inversely, descriptions are `null` (and thus omitted) if they don't exist at all (ex. the `variant_parameter_description` for a method without variant parameters).
/// - Every list is sorted alphabetically by name. To replicate the order seen on the website, it can be sorted by the `order` property of its members.
/// - Text (descriptions, examples, etc.) is formatted as [Markdown](https://daringfireball.net/projects/markdown/), which includes links, inline code, and code blocks. More on how links work right below.
///
/// ## [Link format](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-runtime.html#link-format)
///
/// All text can contain Markdown-type links. There are two broad categories for these: hyperlinks that link to any external website, and internal links that refer to another part of this documentation. All of them will have a title that should be displayed as the link's text.
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
pub struct RuntimeApiFormat {
    /// The application this documentation is for. Will always be `"factorio"`.
    application: String,
    /// Indicates the stage this documentation is for. Will always be `"runtime"` (as opposed to `"prototype"`; see the [Data Lifecycle](https://lua-api.factorio.com/1.1.101/auxiliary/data-lifecycle.html) for more detail).
    stage: String,
    /// The version of the game that this documentation is for. An example would be `"1.1.35"`.
    application_version: String,
    /// The version of the machine-readable format itself. It is incremented every time the format changes. The version this documentation reflects is stated at the top.
    api_version: u8,
    /// The list of classes (LuaObjects) the API provides. Equivalent to the [classes](https://lua-api.factorio.com/1.1.101/classes.html) page.
    classes: Vec<Class>,
    /// The list of events that the API provides. Equivalent to the [events](https://lua-api.factorio.com/1.1.101/events.html) page.
    events: Vec<Event>,
    /// The list of defines that the game uses. Equivalent to the [defines](https://lua-api.factorio.com/1.1.101/defines.html) page.
    defines: Vec<Define>,
    /// The list of types that are built into Lua itself. Equivalent to the [built-in types](https://lua-api.factorio.com/1.1.101/builtin-types.html) page.
    builtin_types: Vec<BuiltinType>,
    /// The list of concepts of various types that the API uses. Equivalent to the [concepts](https://lua-api.factorio.com/1.1.101/concepts.html) page.
    concepts: Vec<Concept>,
    /// The list of objects that the game provides as global variables to serve as entry points to the API.
    global_objects: Vec<GlobalObject>,
    /// The list of functions that the game provides as global variables to provide some specific functionality.
    global_functions: Vec<Method>,
}

impl RuntimeApiFormat {
    pub fn generate_runtime_api(
        &self,
        classes_path: &str,
        events_path: &str,
        concepts_path: &str,
        defines_path: &str,
    ) -> io::Result<()> {
        fs::write(classes_path, generate(&self.classes))?;
        fs::write(events_path, generate(&self.events))?;
        fs::write(concepts_path, generate(&self.concepts))?;
        fs::write(defines_path, generate(&self.defines))?;
        Ok(())
    }

    pub fn generate_factorio_types(&self) -> String {
        let mut result = String::from("pub enum Class {\n");
        for class in &self.classes {
            result.push_str(&format!(
                "    {}(super::classes::{}),\n",
                class.name, class.name
            ));
        }
        result.push_str("}\n\npub enum Concept {\n");
        for concept in &self.concepts {
            result.push_str(&format!(
                "    {}(super::concepts::{}),\n",
                concept.name, concept.name
            ));
        }
        result.push_str("}\n\npub enum Define {\n");
        for define in &self.defines {
            let name = define.name.to_pascal_case();
            result.push_str(&format!("    {}(super::defines::{}),\n", name, name));
        }
        result.push_str("}\n\npub enum Event {\n");
        for event in &self.events {
            let name = event.name.to_pascal_case();
            result.push_str(&format!("    {}(super::events::{}),\n", name, name));
        }
        result.push_str("}\n\n");
        result
    }
}
