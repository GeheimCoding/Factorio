#![allow(unused)]
use std::{
    collections::{HashMap, HashSet},
    fs, io,
};

use serde::Deserialize;

use crate::generator::{
    generate, generate_macros,
    prototype::concept,
    type_::{self, ComplexType, Type},
    Import, Macro, StringTransformation,
};

use super::{
    attribute::{self, Attribute},
    builtin_type::BuiltinType,
    class::Class,
    concept::Concept,
    define::Define,
    event::Event,
    global_object::GlobalObject,
    method::Method,
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
        class_names: &HashSet<String>,
    ) -> io::Result<()> {
        fs::write(
            classes_path,
            generate(
                &self.classes,
                vec![
                    Import::HashMap,
                    Import::Defines,
                    Import::Concepts,
                    Import::MaybeCycle,
                ],
                class_names,
            ),
        )?;
        fs::write(
            events_path,
            generate(
                &self.events,
                vec![
                    Import::HashMap,
                    Import::Defines,
                    Import::Classes,
                    Import::Concepts,
                    Import::MaybeCycle,
                ],
                class_names,
            ),
        )?;
        fs::write(
            concepts_path,
            generate(
                &self.concepts,
                vec![
                    Import::HashMap,
                    Import::Defines,
                    Import::Classes,
                    Import::MaybeCycle,
                ],
                class_names,
            ),
        )?;
        fs::write(defines_path, generate(&self.defines, vec![], class_names))?;
        Ok(())
    }

    pub fn generate_factorio_types(&self) -> String {
        let mut result = generate_macros(vec![Macro::DebugDeserialize, Macro::TagSerdeTag]);
        result.push_str("pub enum Class {\n");
        for class in &self.classes {
            result.push_str(&format!(
                "    {}(super::classes::{}),\n",
                class.name, class.name
            ));
        }
        result.push_str(&format!(
            "}}\n\n{}pub enum Concept {{\n",
            generate_macros(vec![
                Macro::DebugDeserialize,
                Macro::RenameSnakeCase,
                Macro::TagSerdeTag,
            ])
        ));
        for concept in &self.concepts {
            result.push_str(&format!(
                "    {}(super::concepts::{}),\n",
                concept.name, concept.name
            ));
        }
        result.push_str(&format!(
            "}}\n\n{}pub enum Define {{\n",
            generate_macros(vec![Macro::DebugDeserialize, Macro::TagSerdeTag]),
        ));
        for define in &self.defines {
            let name = define.name.to_pascal_case();
            let type_ = if name == "Command" {
                "CommandDefine".to_owned()
            } else if name == "DifficultySettings" {
                "DifficultySettingsDefine".to_owned()
            } else {
                name.clone()
            };
            result.push_str(&format!("    {}(super::defines::{}),\n", name, type_));
        }
        result.push_str(&format!(
            "}}\n\n{}pub enum Event {{\n",
            generate_macros(vec![
                Macro::DebugDeserialize,
                Macro::RenameSnakeCase,
                Macro::TagSerdeTag,
            ])
        ));
        for event in &self.events {
            let name = event.name.to_pascal_case();
            result.push_str(&format!("    {}(super::events::{}),\n", name, name));
        }
        result.push_str("}\n\n");
        result
    }

    pub fn get_class_names(&self) -> HashSet<String> {
        self.classes
            .iter()
            .map(|class| class.name.clone())
            .collect()
    }

    pub fn generate_subclasses(&self, subclasses_path: &str) -> io::Result<()> {
        let mut result = String::from("global.lua_objects.subclasses = {\n");
        for class in &self.classes {
            let mut subclasses = vec![];
            for attribute in &class.attributes {
                if let Some(classes) = attribute.subclasses.as_ref() {
                    subclasses.push((
                        attribute.name.clone(),
                        classes
                            .iter()
                            .map(|class| format!("[\"{class}\"] = 0"))
                            .collect::<Vec<_>>()
                            .join(", "),
                    ));
                }
            }
            if !subclasses.is_empty() {
                result.push_str(&format!("{} = {{\n", class.name));
                for (attribute, subclasses) in subclasses {
                    result.push_str(&format!("    {attribute} = {{{subclasses}}},\n"))
                }
                result.push_str("},\n");
            }
        }
        result.push_str("}\n");
        fs::write(subclasses_path, result)
    }

    pub fn generate_dictionaries(&self, dictionaries_path: &str) -> io::Result<()> {
        let mut result = String::from("global.lua_objects.dictionaries = {\n");
        for class in &self.classes {
            let mut dictionaries = vec![];
            for attribute in &class.attributes {
                let is_dictionary = match &attribute.type_ {
                    Type::Simple(_) => false,
                    Type::Complex(complex) => match complex.as_ref() {
                        ComplexType::Dictionary { key, value }
                        | ComplexType::LuaCustomTable { key, value } => true,
                        _ => false,
                    },
                };
                if is_dictionary {
                    dictionaries.push(attribute.name.clone());
                }
            }
            if !dictionaries.is_empty() {
                result.push_str(&format!("{} = {{\n", class.name));
                for attribute in dictionaries {
                    result.push_str(&format!("    {attribute} = 0,\n"))
                }
                result.push_str("},\n");
            }
        }
        result.push_str("}\n");
        fs::write(dictionaries_path, result)
    }

    pub fn generate_settings_attributes(&self, settings_path: &str) -> io::Result<()> {
        let concepts = self
            .concepts
            .iter()
            .map(|c| (c.name.clone(), c))
            .collect::<HashMap<_, _>>();
        let settings = vec![
            ("LuaMapSettings", "MapSettings"),
            ("LuaDifficultySettings", "DifficultySettings"),
            ("LuaGameViewSettings", "GameViewSettings"),
            (
                "LuaMapSettings.enemy_evolution",
                "EnemyEvolutionMapSettings",
            ),
            (
                "LuaMapSettings.enemy_expansion",
                "EnemyExpansionMapSettings",
            ),
            ("LuaMapSettings.path_finder", "PathFinderMapSettings"),
            ("LuaMapSettings.pollution", "PollutionMapSettings"),
            ("LuaMapSettings.steering", "SteeringMapSettings"),
            ("LuaMapSettings.steering.default", "SteeringMapSetting"),
            ("LuaMapSettings.steering.moving", "SteeringMapSetting"),
            ("LuaMapSettings.unit_group", "UnitGroupMapSettings"),
        ];
        let mut result = String::new();
        for (object_name, concept_name) in settings {
            let concept = concepts.get(concept_name).unwrap();
            result.push_str(&format!(
                "global.lua_objects.attributes[\"{object_name}\"] = {{\n"
            ));
            for attribute in match &concept.type_ {
                Type::Simple(_) => vec![],
                Type::Complex(complex) => match complex.as_ref() {
                    ComplexType::LuaStruct { attributes } => {
                        attributes.iter().map(|a| a.name.clone()).collect()
                    }
                    ComplexType::Table {
                        parameters,
                        variant_parameter_groups,
                        variant_parameter_description,
                    } => parameters
                        .iter()
                        .map(|p| p.name.as_ref().unwrap().clone())
                        .collect(),
                    _ => vec![],
                },
            } {
                result.push_str(&format!("    {attribute} = 0,\n"));
            }
            result.push_str("}\n");
        }
        fs::write(settings_path, result)
    }
}
