#![allow(dead_code)]

use std::{
    collections::HashSet,
    fs,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
    process::Command,
};

use serde::Deserialize;

const UNTAGGED: &str = "#[serde(untagged)]\n";
const RENAME_ALL_KEBAB_CASE: &str = "#[serde(rename_all = \"kebab-case\")]\n";
const DERIVE: &str = "#[derive(Debug, Deserialize)]\n";
const DERIVE_REPR: &str = "#[derive(Debug, Deserialize_repr)]\n#[repr(u8)]\n";
const DERIVE_WITH_HASH: &str = "#[derive(Debug, Deserialize, Eq, PartialEq, Hash)]\n";
const RENAME_WITH_TAG: &str =
    "#[serde(rename_all = \"snake_case\")]\n#[serde(tag = \"serde_tag\")]\n";

trait PascalCase {
    fn to_pascal_case(&self) -> String;
}

trait SnakeCase {
    fn to_snake_case(&self) -> String;
}

trait Description {
    fn to_rust_doc(&self) -> String;
}

trait NotesExamples {
    fn get_notes(&self) -> Option<Vec<String>>;
    fn get_examples(&self) -> Option<Vec<String>>;
}

fn generate_notes_and_examples<T: NotesExamples>(
    interface: &T,
    first_description: bool,
    indent: bool,
) -> String {
    let mut description = String::new();
    let notes = interface.get_notes();
    let examples = interface.get_examples();
    let has_notes = notes.is_some();
    let has_examples = examples.is_some();
    let indent = if indent { "    " } else { "" };

    if !first_description && (has_notes || has_examples) {
        description.push_str(&format!("{indent}///\n"));
    }
    if let Some(notes) = notes {
        description.push_str(&format!("{indent}/// # Notes\n{indent}///\n"));
        for note in notes {
            description.push_str(&format!("{indent}/// * {}\n", note.trim_end()));
        }
    }
    if let Some(examples) = examples {
        if has_notes {
            description.push_str(&format!("{indent}///\n"));
        }
        description.push_str(&format!("{indent}/// # Examples\n{indent}///\n"));
        for example in examples {
            let mut mark_code_block = true;
            for (i, line) in example.split('\n').enumerate() {
                if line.trim_end().is_empty() {
                    description.push_str(&format!("{indent}///\n"));
                    continue;
                }
                description.push_str(&format!("{indent}/// "));
                if i == 0 {
                    description.push_str("* ");
                }
                let mut line = line.trim_end().to_owned();
                if line.ends_with("```") {
                    if mark_code_block {
                        line.push_str("text");
                    }
                    mark_code_block = !mark_code_block;
                }
                description.push_str(&format!("{}\n", line));
            }
        }
    }
    description
}

impl PascalCase for String {
    fn to_pascal_case(&self) -> String {
        if self.is_empty() {
            return String::new();
        }
        let mut chars = self.chars();
        let mut pascal_case = String::from(
            chars
                .next()
                .expect("there should be at least one character")
                .to_ascii_uppercase(),
        );
        while let Some(c) = chars.next() {
            if c == '_' || c == '.' || c == '-' {
                if let Some(next) = chars.next() {
                    pascal_case.push(next.to_ascii_uppercase());
                }
            } else {
                pascal_case.push(c);
            }
        }
        pascal_case
    }
}

impl SnakeCase for String {
    fn to_snake_case(&self) -> String {
        if self.is_empty() {
            return String::new();
        }
        let mut chars = self.chars();
        let mut snake_case = String::from(
            chars
                .next()
                .expect("there should be at least one character")
                .to_ascii_lowercase(),
        );
        for c in chars {
            if c.is_ascii_uppercase() {
                snake_case.push('_');
                snake_case.push(c.to_ascii_lowercase());
            } else {
                snake_case.push(c);
            }
        }
        snake_case
    }
}

impl Description for String {
    fn to_rust_doc(&self) -> String {
        let mut description = String::new();
        let mut mark_code_block = true;
        for line in self.lines() {
            if line.is_empty() {
                description.push_str("///\n");
            } else {
                let mut line = line.trim_end().to_owned();
                if line == "```" {
                    if mark_code_block {
                        line.push_str("text");
                    }
                    mark_code_block = !mark_code_block;
                }
                description.push_str(&format!("/// {}\n", line));
            }
        }
        description
    }
}

fn get_rust_name(name: &str, order: u8) -> (String, bool) {
    if name == "type" {
        ("typ".to_owned(), true)
    } else if name == "mod" {
        ("mod_name".to_owned(), true)
    } else if name == "noisePersistence" {
        ("noise_persistence".to_owned(), true)
    } else if name == "_" {
        (format!("field_{}", order), true)
    } else {
        (name.replace('-', "_").to_owned(), false)
    }
}

#[derive(Debug, Deserialize)]
pub struct RuntimeApiFormat {
    /// The application this documentation is for. Will always be "factorio".
    application: String,
    /// Indicates the stage this documentation is for. Will always be "runtime" (as opposed to "data"; see the Data Lifecycle for more detail).
    stage: String,
    /// The version of the game that this documentation is for. An example would be "1.1.30".
    application_version: String,
    /// The version of the machine-readable format itself. It is incremented every time the format changes. The version this documentation reflects is stated at the top.
    api_version: u8,
    /// The list of classes (LuaObjects) the API provides. Equivalent to the classes page.
    classes: Vec<Class>,
    /// The list of events that the API provides. Equivalent to the events page.
    events: Vec<Event>,
    /// The list of defines that the game uses. Equivalent to the defines page.
    defines: Vec<Define>,
    /// The list of types that are built into Lua itself. Equivalent to the built-in types page.
    builtin_types: Vec<BuiltinType>,
    /// The list of concepts of various types that the API uses. Equivalent to the concepts page.
    concepts: Vec<Concept>,
    /// The list of objects that the game provides as global variables to serve as entry points to the API.
    global_objects: Vec<GlobalObject>,
    /// The list of functions that the game provides as global variables to provide some specific functionality.
    global_functions: Vec<Method>,
}

enum Import {
    HashMap,
    HashSet,
    Classes,
    Concepts,
    Defines,
    Super,
    Deserialize,
    DeserializeRepr,
    LineBreak,
}

trait GenerateDefinition {
    fn generate_definition(&self) -> String;
}

impl RuntimeApiFormat {
    pub fn generate_runtime_api(&self) -> std::io::Result<()> {
        let generated_path = PathBuf::from("src/generated");
        let mut mod_content = String::new();
        fs::create_dir_all(&generated_path)?;

        self.generate_classes(&generated_path, &mut mod_content)?;
        self.generate_events(&generated_path, &mut mod_content)?;
        self.generate_concepts(&generated_path, &mut mod_content)?;
        self.generate_defines(&generated_path, &mut mod_content)?;

        let factorio_types_path = generated_path.join("factorio_types.rs");
        if factorio_types_path.exists() {
            fs::remove_file(&factorio_types_path)?;
        }
        let mut content =
            "use serde::Deserialize;\n\nuse super::classes::*;\nuse super::concepts::*;\nuse super::events::*;\n\n".to_owned();
        content.push_str(&self.generate_factorio_types());
        fs::write(factorio_types_path, content)?;

        let mod_path = generated_path.join("mod.rs");
        mod_content.push_str("mod factorio_types;\npub use factorio_types::*;\n\n");
        if mod_path.exists() {
            fs::remove_file(&mod_path)?;
        }
        mod_content.push_str("use serde::Deserialize;\n\n");
        mod_content.push_str(
            &fs::read_to_string(format!("runtime_api_format/patches/maybe_cycle.rs"))
                .unwrap_or_default(),
        );
        mod_content.push('\n');
        mod_content.push_str(
            &fs::read_to_string(format!("runtime_api_format/patches/floating_point.rs"))
                .unwrap_or_default(),
        );
        fs::write(mod_path, mod_content)?;

        Ok(())
    }

    fn generate_factorio_types(&self) -> String {
        let mut definition = String::from(DERIVE);

        definition
            .push_str("#[serde(rename_all = \"snake_case\")]\n#[serde(tag = \"serde_type\")]\n");
        definition.push_str("pub enum FactorioType {\n    Class(Class),\n    Concept(Concept),\n    Event(Event),\n",);

        definition.push_str(&format!("}}\n\n{DERIVE}#[serde(tag = \"serde_tag\")]\n"));
        definition.push_str("pub enum Class {\n");
        for class in &self.classes {
            let name = &class.name;
            definition.push_str(&format!("    {name}({name}),\n"));
        }
        definition.push_str(&format!(
            "{}{}{}{}{}{}",
            "    LuaEntityBuildFlowStatistics(LuaEntityBuildFlowStatistics),\n",
            "    LuaFluidProductionFlowStatistics(LuaFluidProductionFlowStatistics),\n",
            "    LuaItemProductionFlowStatistics(LuaItemProductionFlowStatistics),\n",
            "    LuaKillCountFlowStatistics(LuaKillCountFlowStatistics),\n",
            "    LuaPollutionFlowStatistics(LuaPollutionFlowStatistics),\n",
            "    LuaItemStackInvalidForRead(LuaItemStackInvalidForRead),\n"
        ));
        definition.push_str(&format!("}}\n\n{DERIVE}{RENAME_WITH_TAG}"));
        definition.push_str("pub enum Concept {\n");
        for concept in &self.concepts {
            let name = &concept.name;
            definition.push_str(&format!("    {name}({name}),\n"));
        }
        definition.push_str(&format!("}}\n\n{DERIVE}{RENAME_WITH_TAG}"));
        definition.push_str("pub enum Event {\n");
        for event in &self.events {
            let name = event.name.to_pascal_case();
            definition.push_str(&format!("    {name}({name}),\n"));
        }
        definition.push_str("}\n");

        definition
    }

    fn generate_definition<T: GenerateDefinition>(
        &self,
        file_path: PathBuf,
        definition_types: &Vec<T>,
        imports: Vec<Import>,
        mod_content: &mut String,
        patch_files: Vec<String>,
    ) -> std::io::Result<()> {
        let file_stem = file_path
            .file_stem()
            .expect("the file should have a stem")
            .to_str()
            .expect("stem should be valid UTF-8");
        let file_name = file_path
            .to_str()
            .expect("file path should be valid UTF-8")
            .to_owned();
        mod_content.push_str(&format!("mod {file_stem};\n"));
        mod_content.push_str(&format!("pub use {file_stem}::*;\n\n"));

        if file_path.exists() {
            fs::remove_file(&file_path)?;
        }
        let mut definition = String::new();
        for import in imports {
            definition.push_str(match import {
                Import::HashMap => "use std::collections::HashMap;\n",
                Import::HashSet => "use std::collections::HashSet;\n",
                Import::Classes => "use super::classes::*;\n",
                Import::Concepts => "use super::concepts::*;\n",
                Import::Defines => "use super::defines::*;\n",
                Import::Super => "use super::*;\n",
                Import::Deserialize => "use serde::Deserialize;\n",
                Import::DeserializeRepr => "use serde_repr::Deserialize_repr;\n",
                Import::LineBreak => "\n",
            });
        }
        for definition_type in definition_types {
            definition.push_str(&format!("{}\n", definition_type.generate_definition()));
        }
        for patch_file in patch_files {
            let patch = fs::read_to_string(format!("runtime_api_format/patches/{patch_file}"))
                .unwrap_or_default();
            definition.push_str(&format!("{patch}\n"));
        }
        definition.pop();
        fs::write(file_path, definition)?;

        Command::new("rustfmt")
            .arg(&file_name)
            .status()
            .map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!("could not format {file_name} due to {e}"),
                )
            })?;

        Ok(())
    }

    fn generate_classes(&self, base_path: &Path, mod_content: &mut String) -> std::io::Result<()> {
        let imports = vec![
            Import::HashMap,
            Import::HashSet,
            Import::LineBreak,
            Import::Deserialize,
            Import::LineBreak,
            Import::Concepts,
            Import::Defines,
            Import::Super,
            Import::LineBreak,
        ];
        self.generate_definition(
            base_path.join("classes.rs"),
            &self.classes,
            imports,
            mod_content,
            vec![
                "lua_entity_build_flow_statistics.rs".to_owned(),
                "lua_fluid_production_flow_statistics.rs".to_owned(),
                "lua_item_production_flow_statistics.rs".to_owned(),
                "lua_kill_count_flow_statistics.rs".to_owned(),
                "lua_pollution_flow_statistics.rs".to_owned(),
                "maybe_lua_item_stack.rs".to_owned(),
            ],
        )
    }

    fn generate_events(&self, base_path: &Path, mod_content: &mut String) -> std::io::Result<()> {
        let imports = vec![
            Import::HashMap,
            Import::LineBreak,
            Import::Deserialize,
            Import::LineBreak,
            Import::Classes,
            Import::Concepts,
            Import::Defines,
            Import::Super,
            Import::LineBreak,
        ];
        self.generate_definition(
            base_path.join("events.rs"),
            &self.events,
            imports,
            mod_content,
            Vec::new(),
        )
    }

    fn generate_concepts(&self, base_path: &Path, mod_content: &mut String) -> std::io::Result<()> {
        let imports = vec![
            Import::HashMap,
            Import::HashSet,
            Import::LineBreak,
            Import::Deserialize,
            Import::LineBreak,
            Import::Classes,
            Import::Defines,
            Import::Super,
            Import::LineBreak,
        ];
        self.generate_definition(
            base_path.join("concepts.rs"),
            &self.concepts,
            imports,
            mod_content,
            Vec::new(),
        )
    }

    fn generate_defines(&self, base_path: &Path, mod_content: &mut String) -> std::io::Result<()> {
        let imports = vec![Import::DeserializeRepr, Import::LineBreak];
        self.generate_definition(
            base_path.join("defines.rs"),
            &self.defines,
            imports,
            mod_content,
            Vec::new(),
        )
    }
}

// Top level types

#[derive(Debug, Deserialize)]
struct Class {
    /// The name of the class.
    name: String,
    /// The order of the class as shown in the html.
    order: u8,
    ///  The text description of the class.
    description: String,
    /// A list of strings containing additional information about the class.
    notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    examples: Option<Vec<String>>,
    /// The methods that are part of the class.
    methods: Vec<Method>,
    /// The attributes that are part of the class.
    attributes: Vec<Attribute>,
    /// A list of operators on the class. They are called call, index, or length and have the format of either a Method or an Attribute.
    operators: Vec<Operator>,
    /// Whether the class is never itself instantiated, only inherited from.
    #[serde(rename = "abstract")]
    abstract_flag: bool,
    /// A list of the names of the classes that his class inherits from.
    base_classes: Option<Vec<String>>,
}

impl NotesExamples for Class {
    fn get_notes(&self) -> Option<Vec<String>> {
        self.notes.clone()
    }

    fn get_examples(&self) -> Option<Vec<String>> {
        self.examples.clone()
    }
}

impl GenerateDefinition for Class {
    fn generate_definition(&self) -> String {
        let mut definition = String::new();
        let mut inline_type_definition = String::new();
        let mut struct_definition = String::new();
        let mut unions = Vec::new();
        let prefix = &self.name;

        struct_definition.push_str(&self.description.to_rust_doc());
        struct_definition.push_str(&generate_notes_and_examples(
            self,
            struct_definition.is_empty(),
            false,
        ));
        struct_definition.push_str(DERIVE);
        struct_definition.push_str(&format!("pub struct {} {{\n", prefix));
        if let Some(base_classes) = &self.base_classes {
            // TODO: there is only one base class here?
            for base_class in base_classes {
                struct_definition.push_str(&format!(
                    "    pub {}: Option<Box<{base_class}>>,\n",
                    base_class.to_snake_case()
                ));
            }
        }
        for attribute in &self.attributes {
            let name = attribute
                .name
                .as_ref()
                .expect("attribute should have a name");
            let prefix = &format!("{}{}", prefix, name.to_pascal_case());
            let typ = Type::lua_type_to_rust_type(&attribute.typ.generate_definition(
                prefix,
                &mut unions,
                true,
            ));
            let typ = if attribute.typ.is_inline_type() {
                inline_type_definition.push_str(&format!("{typ}\n\n"));
                prefix
            } else {
                &typ
            };
            let typ =
                if typ.starts_with("Lua") && !typ.ends_with("Filter") && !typ.ends_with("Union") {
                    format!("MaybeCycle<{typ}>")
                } else {
                    typ.to_owned()
                };
            let mut typ = if attribute.optional {
                format!("Option<{typ}>")
            } else {
                typ
            };
            let (rust_name, needs_rename) = get_rust_name(name, self.order);
            if rust_name == "ended_in_water_trigger_effect" || rust_name == "regular_trigger_effect"
            {
                typ = "Option<Vec<TriggerEffectItem>>".to_owned();
            } else if rust_name == "flags" && self.name == "LuaItemPrototype" {
                typ = "Option<ItemPrototypeFlags>".to_owned();
            } else if rust_name == "flags" && self.name == "LuaEntityPrototype" {
                typ = "Option<EntityPrototypeFlags>".to_owned();
            } else if (rust_name == "center"
                || rust_name == "goal"
                || rust_name == "left"
                || rust_name == "relative"
                || rust_name == "screen"
                || rust_name == "top")
                && self.name == "LuaGui"
            {
                typ = format!("Option<{typ}>");
            } else if self.name == "LuaStyle"
                && !(rust_name == "gui"
                    || rust_name == "horizontal_align"
                    || rust_name == "horizontally_squashable"
                    || rust_name == "horizontally_stretchable"
                    || rust_name == "vertical_align"
                    || rust_name == "vertically_squashable"
                    || rust_name == "vertically_stretchable")
            {
                typ = format!("Option<{typ}>");
            } else if self.name == "LuaPlayer"
                && (rust_name == "blueprint_to_setup" || rust_name == "cursor_stack")
            {
                // TODO: find solution for base class members
                typ = "MaybeCycle<MaybeLuaItemStack>".to_owned();
            } else if self.name == "LuaPlayer"
                && (rust_name == "infinity_inventory_filters"
                    || rust_name == "map_view_settings"
                    || rust_name == "remove_unfiltered_items"
                    || rust_name == "zoom")
            {
                typ = format!("Option<{typ}>");
            } else if rust_name == "difficulty_settings" {
                typ = format!("Option<{typ}>");
            } else if self.name == "LuaGuiElement"
                && !(rust_name == "caption"
                    || rust_name == "children"
                    || rust_name == "children_names"
                    || rust_name == "enabled"
                    || rust_name == "gui"
                    || rust_name == "ignored_by_interaction"
                    || rust_name == "index"
                    || rust_name == "name"
                    || rust_name == "location"
                    || rust_name == "force"
                    || rust_name == "entity"
                    || rust_name == "elem_value"
                    || rust_name == "elem_filters"
                    || rust_name == "drag_target"
                    || rust_name == "anchor"
                    || rust_name == "object_name"
                    || rust_name == "player_index"
                    || rust_name == "raise_hover_events"
                    || rust_name == "style"
                    || rust_name == "tags"
                    || rust_name == "tooltip"
                    || rust_name == "type"
                    || rust_name == "valid"
                    || rust_name == "visible")
            {
                typ = format!("Option<{typ}>");
            } else if self.name == "LuaEntity"
                && (rust_name == "cliff_orientation"
                    || rust_name == "corpse_expires"
                    || rust_name == "corpse_immune_to_entity_placement"
                    || rust_name == "drop_position"
                    || rust_name == "inserter_stack_size_override"
                    || rust_name == "inserter_target_pickup_count"
                    || rust_name == "item_requests"
                    || rust_name == "link_id"
                    || rust_name == "logistic_cell"
                    || rust_name == "logistic_network"
                    || rust_name == "neighbours"
                    || rust_name == "power_switch_state"
                    || rust_name == "request_from_buffers"
                    || rust_name == "rocket_silo_status"
                    || rust_name == "sticked_to"
                    || rust_name == "storage_filter"
                    || rust_name == "time_to_live"
                    || rust_name == "units"
                    || rust_name.starts_with("tree_"))
            {
                typ = format!("Option<{typ}>");
            }

            let mut attribute_description = String::new();
            if !attribute.description.is_empty() {
                let descriptions = attribute.description.split('\n');
                for description in descriptions {
                    if description.is_empty() {
                        attribute_description.push_str("    ///\n");
                    } else {
                        attribute_description
                            .push_str(&format!("    /// {}\n", description.trim_end()));
                    }
                }
            }
            if let Some(subclasses) = &attribute.subclasses {
                let mut subclass_description = String::from("    /// Can only be used if this is ");
                subclass_description.push_str(&subclasses.join(" or "));
                subclass_description.push('\n');
                attribute_description.push_str(&subclass_description);
                // TODO: patch the spec instead?
                if !typ.starts_with("Option<") {
                    typ = format!("Option<{typ}>");
                }
            }
            attribute_description.push_str(&generate_notes_and_examples(
                attribute,
                attribute_description.is_empty(),
                true,
            ));
            struct_definition.push_str(&attribute_description);
            if needs_rename {
                struct_definition.push_str(&format!("    #[serde(rename = \"{name}\")]\n"));
            }
            struct_definition.push_str(&format!("    pub {}: {},\n", rust_name, typ));
        }
        struct_definition.push_str("}\n");
        for union in unions {
            definition.push_str(&format!("{}\n\n", union));
        }
        definition.push_str(&inline_type_definition);
        definition.push_str(&struct_definition);
        definition.push_str(&self.generate_methods(prefix));

        definition
    }
}

impl Class {
    fn generate_methods(&self, prefix: &str) -> String {
        let mut definition = String::from('\n');
        let mut methods = String::new();
        let descriptions = self.description.split('\n');
        for desciption in descriptions {
            if desciption.is_empty() {
                methods.push_str("///\n");
            } else {
                methods.push_str(&format!("/// {}\n", desciption.trim_end()));
            }
        }
        let prefix = format!("{prefix}Methods");
        let mut unions = Vec::new();
        methods.push_str(&format!("pub trait {prefix} {{\n"));
        // each class has at least one method
        for method in &self.methods {
            methods.push_str(&method.generate_definition(&prefix, &mut unions));
        }
        methods.push_str("}\n");

        for union in unions {
            definition.push_str(&format!("{union}\n\n"));
        }

        definition.push_str(&methods);
        definition
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "name")]
enum Operator {
    Call(Method),
    Index(Attribute),
    Length(Attribute),
}

#[derive(Debug, Deserialize)]
struct Event {
    /// The name of the event.
    name: String,
    /// The order of the event as shown in the html.
    order: u8,
    /// The text description of the event.
    description: String,
    /// A list of strings containing additional information about the event.
    notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    examples: Option<Vec<String>>,
    /// The event-specific information that is provided.
    data: Vec<Parameter>,
}

impl NotesExamples for Event {
    fn get_notes(&self) -> Option<Vec<String>> {
        self.notes.clone()
    }

    fn get_examples(&self) -> Option<Vec<String>> {
        self.examples.clone()
    }
}

impl GenerateDefinition for Event {
    fn generate_definition(&self) -> String {
        let mut definition = String::new();
        definition.push_str(&self.description.to_rust_doc());
        definition.push_str(&generate_notes_and_examples(
            self,
            definition.is_empty(),
            false,
        ));
        definition.push_str(DERIVE);
        definition.push_str(&format!("pub struct {} {{\n", self.name.to_pascal_case()));
        for parameter in &self.data {
            definition.push_str(&parameter.get_definition(&mut Vec::new(), ""));
        }
        definition.push_str("}\n");
        definition
    }
}

/// Defines can be recursive in nature, meaning one Define can have multiple sub-Defines that have the same structure. These are singled out as subkeys instead of values.
#[derive(Debug, Deserialize)]
struct Define {
    /// The name of the define.
    name: String,
    /// The order of the define as shown in the html.
    order: u8,
    /// The text description of the define.
    description: String,
    /// The members of the define.
    values: Option<Vec<BasicMember>>,
    /// A list of sub-defines.
    subkeys: Option<Vec<Define>>,
}

impl GenerateDefinition for Define {
    fn generate_definition(&self) -> String {
        self.generate_definition_internal("")
    }
}

impl Define {
    fn generate_definition_internal(&self, prefix: &str) -> String {
        let mut definition = String::new();
        let name = &format!("{}{}", prefix, self.name.to_pascal_case());

        // either we have variants
        if let Some(variants) = &self.values {
            if self.subkeys.is_some() {
                unreachable!();
            }
            let name = if name == "Command" {
                "CommandDefine"
            } else {
                name
            };
            definition.push_str(&self.description.to_rust_doc());
            definition.push_str(DERIVE_REPR);
            definition.push_str(&format!("pub enum {} {{\n", name));
            for variant in variants {
                definition.push_str(&variant.generate_definition());
            }
            definition.push('}');
        // or sub-defines
        } else if let Some(sub_defines) = &self.subkeys {
            if self.values.is_some() {
                unreachable!();
            }
            for sub_define in sub_defines {
                definition.push_str(&format!(
                    "{}\n",
                    &sub_define.generate_definition_internal(name)
                ));
            }
            definition.pop();
        // or an empty struct
        } else {
            definition.push_str(&self.description.to_rust_doc());
            definition.push_str(&format!("pub struct {};\n", name));
        }
        if !definition.ends_with('\n') {
            definition.push('\n');
        }
        definition
    }
}

#[derive(Debug, Deserialize)]
struct BuiltinType {
    /// The name of the builtin type.
    name: String,
    /// The order of the builtin type as shown in the html.
    order: u8,
    /// The text description of the builtin type.
    description: String,
}

#[derive(Debug, Deserialize)]
struct Concept {
    /// The name of the concept.
    name: String,
    /// The order of the concept as shown in the html.
    order: u8,
    /// The text description of the concept.
    description: String,
    /// A list of strings containing additional information about the concept.
    notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    examples: Option<Vec<String>>,
    /// The type of the concept.
    #[serde(rename = "type")]
    typ: Type,
}

impl NotesExamples for Concept {
    fn get_notes(&self) -> Option<Vec<String>> {
        self.notes.clone()
    }

    fn get_examples(&self) -> Option<Vec<String>> {
        self.examples.clone()
    }
}

impl GenerateDefinition for Concept {
    fn generate_definition(&self) -> String {
        let mut definition = String::new();
        let mut unions = Vec::new();
        let mut type_definition = self.typ.generate_definition(&self.name, &mut unions, false);
        for union in unions {
            definition.push_str(&format!("{}\n\n", &union));
        }
        let mut description = self.description.to_rust_doc();
        description.push_str(&generate_notes_and_examples(
            self,
            description.is_empty(),
            false,
        ));
        if self.name == "MapPosition" {
            type_definition = fs::read_to_string("runtime_api_format/patches/map_position.rs")
                .unwrap_or_default();
        } else if self.name == "RenderLayer" {
            type_definition = fs::read_to_string("runtime_api_format/patches/render_layer.rs")
                .unwrap_or_default();
        } else if self.name == "CollisionMaskLayer" {
            type_definition =
                fs::read_to_string("runtime_api_format/patches/collision_mask_layer.rs")
                    .unwrap_or_default();
        }
        let position = type_definition.rfind("pub struct");
        let position = if let Some(position) = position {
            position
        } else {
            0
        };
        type_definition.insert_str(position, &description);
        definition.push_str(&format!("{}\n", &type_definition));
        definition
    }
}

#[derive(Debug, Deserialize)]
struct GlobalObject {
    /// The global variable name of the object.
    name: String,
    /// The order of the global object as shown in the html.
    order: u8,
    /// The text description of the global object.
    description: String,
    /// The class name of the global object.
    #[serde(rename = "type")]
    typ: String,
}

// Common structures

#[derive(Debug, Deserialize)]
struct BasicMember {
    /// The name of the member.
    name: String,
    /// The order of the member as shown in the html.
    order: u8,
    /// The text description of the member.
    description: String,
}

impl GenerateDefinition for BasicMember {
    fn generate_definition(&self) -> String {
        let mut description = String::new();
        if !self.description.is_empty() {
            description.push_str(&format!("    /// {}\n", self.description.trim_end()));
        }
        description.push_str(&format!("    {},\n", self.name.to_pascal_case()));
        description
    }
}

#[derive(Debug, Deserialize)]
struct EventRaised {
    /// The name of the event being raised.
    name: String,
    /// The order of the member as shown in the html.
    order: u8,
    /// The text description of the raised event.
    description: String,
    /// The timeframe during which the event is raised. One of "instantly", "current_tick", or "future_tick".
    timeframe: String,
    /// Whether the event is always raised, or only dependant on a certain condition.
    optional: bool,
}

/// A type is either a string, in which case that string is the simple type. Otherwise, a type is a table:
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
enum Type {
    String(String),
    ComplexType(ComplexType),
}

impl Type {
    fn is_inline_type(&self) -> bool {
        match self {
            Self::String(_) => false,
            Self::ComplexType(complex_type) => complex_type.is_inline_type(),
        }
    }

    fn is_owned_type(&self) -> bool {
        if let Type::ComplexType(ComplexType::Literal {
            value: _,
            description: _,
        }) = self
        {
            true
        } else {
            matches!(self.get_type_name().as_str(), "table" | "nil" | "LuaObject")
        }
    }

    fn lua_type_to_rust_type(type_name: &str) -> String {
        match type_name {
            "float" => "Float".to_owned(),
            "double" => "Double".to_owned(),
            "int" => "i32".to_owned(),
            "int8" => "i8".to_owned(),
            "uint" => "u32".to_owned(),
            "uint8" => "u8".to_owned(),
            "uint16" => "u16".to_owned(),
            "uint64" => "u64".to_owned(),
            "number" => "f64".to_owned(),
            "string" => "String".to_owned(),
            "boolean" => "bool".to_owned(),
            name if name.starts_with("defines.") => {
                let mut name = name[8..].to_owned().to_pascal_case();
                if name == "Command" {
                    name.push_str("Define")
                }
                name
            }
            name => name.to_owned(),
        }
    }
}

/// Depending on complex_type, there are additional members:
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "complex_type")]
enum ComplexType {
    Type {
        /// The actual type. This format for types is used when they have descriptions attached to them.
        value: Box<Type>,
        /// The text description of the type.
        description: String,
    },
    Union {
        /// A list of all compatible types for this type.
        options: Vec<Type>,
        /// Whether the options of this union have a description or not.
        full_format: bool,
    },
    Array {
        /// The type of the elements of the array.
        value: Box<Type>,
    },
    Dictionary {
        /// The type of the keys of the dictionary.
        key: Box<Type>,
        /// The type of the values of the dictionary.
        value: Box<Type>,
    },
    #[serde(rename = "LuaCustomTable")]
    LuaCustomTable {
        /// The type of the keys of the LuaCustomTable.
        key: Box<Type>,
        /// The type of the values of the LuaCustomTable.
        value: Box<Type>,
    },
    Function {
        /// The types of the function arguments.
        parameters: Vec<Type>,
    },
    Literal {
        /// The value of the literal.
        value: LiteralValue,
        /// The text description of the literal, if any.
        description: Option<String>,
    },
    #[serde(rename = "LuaLazyLoadedValue")]
    LuaLazyLoadedValue {
        /// The type of the LuaLazyLoadedValue.
        value: Box<Type>,
    },
    Struct {
        /// A list of attributes with the same properties as class attributes.
        attributes: Vec<Attribute>,
    },
    Table {
        /// The parameters present in the table.
        parameters: Vec<Parameter>,
        /// The optional parameters that depend on one of the main parameters.
        variant_parameter_groups: Option<Vec<ParameterGroup>>,
        /// The text description of the optional parameter groups.
        variant_parameter_description: Option<String>,
    },
    Tuple {
        /// The parameters present in the table.
        parameters: Vec<Parameter>,
        /// The optional parameters that depend on one of the main parameters.
        variant_parameter_groups: Option<Vec<ParameterGroup>>,
        /// The text description of the optional parameter groups.
        variant_parameter_description: Option<String>,
    },
}

impl ComplexType {
    fn is_inline_type(&self) -> bool {
        matches!(
            self,
            Self::Table {
                parameters: _,
                variant_parameter_groups: _,
                variant_parameter_description: _,
            } | Self::Tuple {
                parameters: _,
                variant_parameter_groups: _,
                variant_parameter_description: _,
            }
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
enum LiteralValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug, Deserialize)]
struct Parameter {
    /// The name of the parameter.
    name: Option<String>,
    /// The order of the parameter as shown in the html.
    order: u8,
    /// The text description of the parameter.
    description: String,
    /// The type of the parameter.
    #[serde(rename = "type")]
    typ: Type,
    /// Whether the type is optional or not.
    optional: bool,
}

impl Parameter {
    fn get_definition(&self, unions: &mut Vec<String>, prefix: &str) -> String {
        let mut definition = String::new();
        let name = self.name.as_ref().expect("parameter should have a name");
        let prefix = &format!("{}{}", prefix, name.to_pascal_case());
        let typ = Type::lua_type_to_rust_type(&self.typ.generate_definition(prefix, unions, true));
        let typ = if typ.starts_with("Lua") && !typ.ends_with("Filter") && !typ.ends_with("Union") {
            format!("MaybeCycle<{typ}>")
        } else {
            typ.to_owned()
        };
        let mut typ = if self.optional || prefix.starts_with("AutoplaceSpecification") {
            format!("Option<{}>", typ)
        } else {
            typ
        };
        let (rust_name, needs_rename) = get_rust_name(name, self.order);
        if !self.description.is_empty() {
            let descriptions = self.description.split('\n');
            for description in descriptions {
                if description.is_empty() {
                    definition.push_str("    ///\n");
                } else {
                    definition.push_str(&format!("    /// {}\n", description.trim_end()));
                }
            }
        }
        if needs_rename {
            definition.push_str(&format!("    #[serde(rename = \"{name}\")]\n"));
        }
        if prefix.starts_with("TriggerEffectItem")
            || rust_name == "default_enable_all_autoplace_controls"
            || (prefix.starts_with("LuaTilePrototypeMineableProperties") && rust_name == "products")
            || prefix.starts_with("AutoplaceControl")
            || (prefix.starts_with("DifficultySettings") && rust_name == "research_queue_setting")
            || rust_name == "enemy_evolution"
            || rust_name == "enemy_expansion"
            || rust_name == "difficulty_settings"
            || prefix.starts_with("MapGenSettings")
            || prefix.starts_with("PollutionMapSettings")
            || prefix.starts_with("EnemyEvolutionMapSettings")
            || prefix.starts_with("EnemyExpansionMapSettings")
            || (prefix.contains("Settings") && rust_name == "pollution")
        {
            typ = format!("Option<{}>", typ);
        }
        definition.push_str(&format!("    pub {}: {},\n", rust_name, typ));
        definition
    }
}

#[derive(Debug, Deserialize)]
struct ParameterGroup {
    /// The name of the parameter group.
    name: String,
    /// The order of the parameter group as shown in the html.
    order: u8,
    /// The text description of the parameter group.
    description: Option<String>,
    /// The parameters that the group adds.
    parameters: Vec<Parameter>,
}

#[derive(Debug, Deserialize)]
struct Method {
    /// The name of the method.
    name: Option<String>,
    /// The order of the method as shown in the html.
    order: u8,
    /// The text description of the method.
    description: String,
    /// A list of strings containing additional information about the method.
    notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    examples: Option<Vec<String>>,
    /// A list of events that this method might raise when called.
    raises: Option<Vec<EventRaised>>,
    /// A list of strings specifying the sub-type (of the class) that the method applies to.
    subclasses: Option<Vec<String>>,
    /// The parameters of the method. How to interpret them depends on the takes_table member.
    parameters: Vec<Parameter>,
    /// The optional parameters that depend on one of the main parameters. Only applies if takes_table is true.
    variant_parameter_groups: Option<Vec<ParameterGroup>>,
    /// The text description of the optional parameter groups.
    variant_parameter_description: Option<String>,
    /// The type of the variadic arguments of the method, if it accepts any.
    variadic_type: Option<Type>,
    /// The description of the variadic arguments of the method, if it accepts any.
    variadic_description: Option<String>,
    /// Whether the method takes a single table with named parameters or a sequence of unnamed parameters.
    takes_table: bool,
    /// If takes_table is true, whether that whole table is optional or not.
    table_is_optional: Option<bool>,
    /// The return values of this method, which can contain zero, one, or multiple values. Note that these have the same structure as parameters, but do not specify a name.
    return_values: Vec<Parameter>,
}

impl NotesExamples for Method {
    fn get_notes(&self) -> Option<Vec<String>> {
        self.notes.clone()
    }

    fn get_examples(&self) -> Option<Vec<String>> {
        self.examples.clone()
    }
}

impl Method {
    fn generate_definition(&self, prefix: &str, unions: &mut Vec<String>) -> String {
        let mut definition = String::new();
        if !self.description.is_empty() {
            let descriptions = self.description.split('\n');
            for desciption in descriptions {
                if desciption.is_empty() {
                    definition.push_str("    ///\n");
                } else {
                    definition.push_str(&format!("    /// {}\n", desciption.trim_end()));
                }
            }
        }
        definition.push_str(&generate_notes_and_examples(
            self,
            definition.is_empty(),
            true,
        ));
        let name = self.name.as_ref().expect("method should have a name");
        let prefix = format!("{prefix}{}", name.to_pascal_case());
        let name = if name == "move" { "mov" } else { name };
        let mut method_definition = String::new();
        let mut argument_descriptions = String::new();
        let mut return_descriptions = String::new();
        let mut has_argument_descriptions = false;
        let mut has_return_descriptions = false;
        method_definition.push_str(&format!("    fn {name}("));

        for (i, parameter) in self.parameters.iter().enumerate() {
            let name = parameter
                .name
                .as_ref()
                .expect("parameter should have a name");
            let (rust_name, _needs_rename) = get_rust_name(name, self.order);
            let prefix = format!("{prefix}{}", rust_name.to_pascal_case());
            let typ = parameter.typ.generate_definition(&prefix, unions, true);
            let typ = if typ == "table" {
                "LuaCustomTable".to_owned()
            } else {
                typ
            };
            if !parameter.description.is_empty() {
                argument_descriptions.push_str(&format!(
                    "    /// * `{rust_name}` - {}\n",
                    parameter.description.trim_end()
                ));
                has_argument_descriptions = true;
            }
            method_definition.push_str(&format!("{}: {}", rust_name, typ));
            if i != self.parameters.len() - 1 {
                method_definition.push_str(", ");
            }
        }

        // TODO: variant_parameter_groups?

        method_definition.push(')');

        let return_values = &self.return_values;
        let multi_return = return_values.len() > 1;
        if !return_values.is_empty() {
            method_definition.push_str(" -> ");
            if multi_return {
                method_definition.push('(');
            }
            for (i, return_value) in return_values.iter().enumerate() {
                let return_type = &Type::lua_type_to_rust_type(
                    &return_value.typ.generate_definition(&prefix, unions, true),
                );
                if !return_value.description.is_empty() {
                    return_descriptions.push_str(&format!(
                        "    /// * {}\n",
                        return_value.description.trim_end()
                    ));
                    has_return_descriptions = true;
                }
                if return_value.optional {
                    method_definition.push_str(&format!("Option<{return_type}>"));
                } else {
                    method_definition.push_str(return_type);
                }
                if i != return_values.len() - 1 {
                    method_definition.push_str(", ");
                }
            }
            if multi_return {
                method_definition.push(')');
            }
        }
        method_definition.push_str(";\n");

        if !definition.is_empty() && (has_argument_descriptions || has_return_descriptions) {
            definition.push_str("    ///\n");
        }
        if has_argument_descriptions {
            definition.push_str("    /// # Arguments\n    ///\n");
            definition.push_str(&argument_descriptions);
        }
        if has_return_descriptions {
            if has_argument_descriptions {
                definition.push_str("    ///\n");
            }
            definition.push_str("    /// # Returns\n    ///\n");
            definition.push_str(&return_descriptions);
        }
        definition.push_str(&method_definition);
        definition
    }
}

#[derive(Debug, Deserialize)]
struct Attribute {
    /// The name of the attribute.
    name: Option<String>,
    /// The order of the attribute as shown in the html.
    order: u8,
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
    typ: Type,
    /// Whether the attribute is optional or not.
    optional: bool,
    /// Whether the attribute can be read from.
    read: bool,
    /// Whether the attribute can be written to.
    write: bool,
}

impl NotesExamples for Attribute {
    fn get_notes(&self) -> Option<Vec<String>> {
        self.notes.clone()
    }

    fn get_examples(&self) -> Option<Vec<String>> {
        self.examples.clone()
    }
}

impl Type {
    fn get_type_name(&self) -> String {
        match self {
            Self::String(string) => string.clone(),
            Self::ComplexType(complex_type) => complex_type.get_type_name(),
        }
    }

    fn get_description(&self) -> Option<String> {
        match self {
            Type::String(_) => None,
            Type::ComplexType(complex_type) => complex_type.get_description(),
        }
    }

    fn generate_definition(
        &self,
        prefix: &str,
        unions: &mut Vec<String>,
        is_nested: bool,
    ) -> String {
        match self {
            Self::String(string) => {
                if string == "BlueprintCircuitConnection" {
                    unions.push(format!("{DERIVE}pub struct BlueprintCircuitConnection;"));
                } else if string == "BlueprintControlBehavior" {
                    unions.push(format!("{DERIVE}pub struct BlueprintControlBehavior;"));
                }
                let mut definition = String::new();
                if !is_nested {
                    definition.push_str(&format!("pub type {prefix} = "));
                }
                definition.push_str(&Type::lua_type_to_rust_type(string));
                if !is_nested {
                    definition.push(';');
                }
                definition
            }
            Self::ComplexType(complex_type) => {
                complex_type.generate_definition(prefix, unions, is_nested)
            }
        }
    }
}

impl ComplexType {
    fn get_type_name(&self) -> String {
        match self {
            Self::Type {
                value,
                description: _,
            } => value.get_type_name(),
            Self::Literal {
                value,
                description: _,
            } => value.get_type_name(),
            Self::Table {
                parameters: _,
                variant_parameter_groups: _,
                variant_parameter_description: _,
            } => "table".to_string(),
            Self::Tuple {
                parameters: _,
                variant_parameter_groups: _,
                variant_parameter_description: _,
            } => "tuple".to_string(),
            Self::Array { value: _ } => "array".to_string(),
            Self::Dictionary { key: _, value: _ } => "dictionary".to_owned(),
            Self::Function { parameters: _ } => "function".to_owned(),
            _ => {
                println!("{self:?}");
                unimplemented!()
            }
        }
    }

    fn get_description(&self) -> Option<String> {
        let description = match self {
            ComplexType::Type {
                value: _,
                description,
            } => Some(description.clone()),
            ComplexType::Literal {
                value: _,
                description,
            } => description.clone(),
            _ => None,
        };
        description.filter(|description| !description.is_empty())
    }

    fn generate_definition(
        &self,
        prefix: &str,
        unions: &mut Vec<String>,
        is_nested: bool,
    ) -> String {
        let mut definition = String::new();
        match self {
            Self::Union {
                options,
                full_format: _,
            } => {
                let mut union_definition = String::new();
                let prefix = if is_nested {
                    format!("{prefix}Union")
                } else {
                    prefix.to_owned()
                };
                if self.is_table_or_tuple() {
                    return options[0].generate_definition(&prefix, unions, true);
                }
                if prefix == "CollisionMaskWithFlagsUnion" {
                    union_definition = fs::read_to_string(
                        "runtime_api_format/patches/collision_mask_with_flags_union.rs",
                    )
                    .unwrap_or_default();
                    unions.push(union_definition);
                    return prefix;
                }
                // TODO: use lazy_static HashSet
                let mut derives = true;
                if prefix == "LuaBootstrapMethodsOnConfigurationChangedHandlerUnion"
                    || prefix == "LuaBootstrapMethodsOnEventHandlerUnion"
                    || prefix == "LuaBootstrapMethodsOnInitHandlerUnion"
                    || prefix == "LuaBootstrapMethodsOnLoadHandlerUnion"
                    || prefix == "LuaBootstrapMethodsOnNthTickHandlerUnion"
                {
                    // we don't want any derives here
                    derives = false;
                } else if prefix == "CollisionMaskWithFlagsUnion"
                    || prefix == "CollisionMaskLayer"
                    || prefix == "EntityPrototypeFlagsUnion"
                    || prefix == "ItemPrototypeFlagsUnion"
                    || prefix == "MouseButtonFlagsUnion"
                    || prefix == "SelectionModeFlagsUnion"
                    || prefix == "LuaGameScriptPlayersUnion"
                    || prefix == "LuaGameScriptForcesUnion"
                    || prefix == "LuaGameScriptSurfacesUnion"
                {
                    union_definition.push_str(DERIVE_WITH_HASH);
                } else {
                    union_definition.push_str(DERIVE);
                }
                if derives {
                    // TODO: detect this with code?
                    if prefix == "ItemPrototypeFlagsUnion"
                        || prefix == "CollisionMaskLayer"
                        || prefix == "EntityPrototypeFlagsUnion"
                        || prefix == "Alignment"
                        || prefix == "CliffOrientation"
                        || prefix == "ComparatorString"
                        || prefix == "CursorBoxRenderType"
                        || prefix == "ForceCondition"
                        || prefix == "MouseButtonFlagsUnion"
                        || prefix == "SelectionModeFlagsUnion"
                        || prefix == "SoundType"
                    {
                        union_definition.push_str(RENAME_ALL_KEBAB_CASE);
                    } else {
                        union_definition.push_str(UNTAGGED);
                    }
                }
                union_definition.push_str(&format!("pub enum {} {{\n", prefix));
                let array_count = options
                    .iter()
                    .map(Type::get_type_name)
                    .filter(|name| name == "array")
                    .count();
                for option in options {
                    let mut type_name = option.get_type_name();
                    let description = option.get_description();
                    if let Some(description) = description {
                        union_definition.push_str(&format!("    /// {}\n", description.trim_end()));
                    }
                    if option.is_owned_type() {
                        if !type_name.is_empty() {
                            let type_name = type_name.to_pascal_case();
                            if type_name == "BuildingDirection8Way" {
                                union_definition
                                    .push_str("#[serde(rename = \"building-direction-8-way\")]\n");
                            }
                            union_definition.push_str(&format!("    {type_name},\n"));
                        }
                    } else {
                        let typ = if type_name == "array" {
                            let array_definition =
                                option.generate_definition(&prefix, unions, true);
                            let extension = &array_definition[4..array_definition.len() - 1];
                            let extension = extension.replace('<', "").replace('>', "");
                            if array_count > 1 {
                                type_name.push('_');
                                type_name.push_str(&extension);
                            }
                            array_definition
                        } else if type_name == "dictionary" || type_name == "function" {
                            option.generate_definition(&prefix, unions, true)
                        } else {
                            Type::lua_type_to_rust_type(&type_name)
                        };
                        let typ = if typ.starts_with("Lua")
                            && !typ.ends_with("Filter")
                            && !typ.ends_with("Union")
                        {
                            format!("MaybeCycle<{typ}>")
                        } else {
                            typ.to_owned()
                        };
                        union_definition.push_str(&format!(
                            "    {}({}),\n",
                            type_name.to_pascal_case(),
                            typ
                        ));
                    }
                }
                union_definition.push('}');

                if is_nested {
                    unions.push(union_definition);
                    definition.push_str(&prefix);
                } else {
                    definition.push_str(&union_definition);
                }
            }
            Self::Table {
                parameters,
                variant_parameter_groups,
                variant_parameter_description,
            }
            | Self::Tuple {
                parameters,
                variant_parameter_groups,
                variant_parameter_description,
            } => {
                let mut table_definition = String::new();
                table_definition.push_str(DERIVE);
                table_definition.push_str(&format!("pub struct {} {{\n", prefix));
                for parameter in parameters {
                    table_definition.push_str(&parameter.get_definition(unions, prefix));
                }
                if let Some(variant_parameter_groups) = variant_parameter_groups {
                    table_definition.push_str(&format!(
                        "    /// {}\n",
                        variant_parameter_description
                            .as_ref()
                            .expect("variant parameter groups should have a description")
                    ));
                    table_definition.push_str(&format!(
                        "    pub attributes: Option<{}Attributes>,\n",
                        prefix
                    ));
                    let mut variant_definition = String::new();
                    let prefix = &format!("{}Attributes", prefix);
                    variant_definition.push_str(DERIVE);
                    variant_definition.push_str(&format!("pub enum {} {{\n", prefix));
                    for variant_parameter_group in variant_parameter_groups {
                        let group_name = variant_parameter_group
                            .name
                            .replace("Other types", "other-types");
                        let name = &format!("{}-{}", prefix, group_name).to_pascal_case();
                        variant_definition.push_str(&format!(
                            "    {}({}),\n",
                            group_name.to_pascal_case(),
                            name
                        ));
                        // TODO: maybe collapse one element types?
                        // if variant_parameter_group.parameters.len() == 1 {
                        //     print!("one for: {name} - ");
                        //     let parameter = &variant_parameter_group.parameters[0];
                        //     let name = parameter.name.as_ref().unwrap().replace("-", "_");
                        //     let prefix = &format!("{}{}", prefix, name.to_pascal_case());
                        //     let typ = Type::lua_type_to_rust_type(
                        //         &parameter.typ.generate_definition(prefix, unions, true),
                        //     );
                        //     println!("{typ}");
                        //     continue;
                        // }
                        definition.push_str(DERIVE);
                        definition.push_str(&format!("pub struct {} {{\n", name));
                        for parameter in &variant_parameter_group.parameters {
                            let name = parameter
                                .name
                                .as_ref()
                                .expect("parameter should have a name")
                                .replace('-', "_");
                            let prefix = &format!("{}{}", prefix, name.to_pascal_case());
                            let typ = Type::lua_type_to_rust_type(
                                &parameter.typ.generate_definition(prefix, unions, true),
                            );
                            let typ = if typ.starts_with("Lua")
                                && !typ.ends_with("Filter")
                                && !typ.ends_with("Union")
                            {
                                format!("MaybeCycle<{typ}>")
                            } else {
                                typ.to_owned()
                            };
                            let typ = if parameter.optional {
                                format!("Option<{typ}>")
                            } else {
                                typ
                            };
                            let (rust_name, needs_rename) = get_rust_name(&name, 0);
                            if !parameter.description.is_empty() {
                                definition.push_str(&format!(
                                    "    /// {}\n",
                                    parameter.description.trim_end()
                                ));
                            }
                            if needs_rename {
                                definition
                                    .push_str(&format!("    #[serde(rename = \"{name}\")]\n"));
                            }
                            definition.push_str(&format!("    pub {}: {},\n", rust_name, typ));
                        }
                        definition.push_str("}\n\n");
                    }
                    variant_definition.push_str("}\n\n");
                    definition.push_str(&variant_definition);
                }
                table_definition.push('}');
                definition.push_str(&table_definition);
            }
            Self::Array { value } => {
                if !is_nested {
                    definition.push_str(&format!("pub type {prefix} = "));
                }
                let typ = value.generate_definition(prefix, unions, true);
                let typ = if typ.starts_with("Lua")
                    && !typ.ends_with("Filter")
                    && !typ.ends_with("Union")
                {
                    format!("MaybeCycle<{typ}>")
                } else {
                    typ.to_owned()
                };
                if typ == "TriggerEffectItem" {
                    definition.push_str("Option<Vec<TriggerEffectItem>>");
                } else {
                    definition.push_str(&format!("Vec<{typ}>"));
                }
                if !is_nested {
                    definition.push(';');
                }
            }
            Self::Dictionary { key, value } | Self::LuaCustomTable { key, value } => {
                if !is_nested {
                    definition.push_str(&format!("pub type {prefix} = "));
                }
                let is_map = true;
                // TODO: enable me and change lua serializer instead?
                // if let Type::ComplexType(ComplexType::Literal {
                //     value,
                //     description: _,
                // }) = value.as_ref()
                // {
                //     // description is always None
                //     if value.get_type_name() == "True" {
                //         definition.push_str(&format!(
                //             "HashSet<{}>",
                //             key.generate_definition(prefix, unions, true)
                //         ));
                //         is_map = false;
                //     }
                // } else if let Type::String(value) = value.as_ref() {
                //     if value == "boolean" {
                //         definition.push_str(&format!(
                //             "HashSet<{}>",
                //             key.generate_definition(prefix, unions, true)
                //         ));
                //         is_map = false;
                //     }
                // }
                if is_map {
                    let value = value.generate_definition(prefix, unions, true);
                    let value = if value.starts_with("Lua")
                        && !value.ends_with("Filter")
                        && !value.ends_with("Union")
                    {
                        format!("MaybeCycle<{value}>")
                    } else if value == "True" {
                        "bool".to_owned()
                    } else {
                        value
                    };
                    let mut key = key.generate_definition(prefix, unions, true);
                    if key == "u32" {
                        key = "String".to_owned();
                    }
                    definition.push_str(&format!("HashMap<{key}, {value}>"));
                }
                if !is_nested {
                    definition.push(';');
                }
            }
            Self::Literal {
                value,
                description: _,
            } => {
                // description is always None
                definition.push_str(&value.get_type_name());
            }
            Self::Struct { attributes } => {
                definition.push_str(DERIVE);
                definition.push_str(&format!("pub struct {} {{\n", prefix));
                for attribute in attributes {
                    let name = attribute
                        .name
                        .as_ref()
                        .expect("attribute should have a name");
                    let prefix = &format!("{}{}", prefix, name.to_pascal_case());
                    let typ = Type::lua_type_to_rust_type(
                        &attribute.typ.generate_definition(prefix, unions, true),
                    );
                    definition.push_str(&format!("    /// {}\n", attribute.description.trim_end()));
                    definition.push_str(&generate_notes_and_examples(
                        attribute,
                        definition.is_empty(),
                        true,
                    ));
                    definition.push_str(&format!("    pub {}: {},\n", name, typ));
                }
                definition.push('}');
            }
            Self::LuaLazyLoadedValue { value } => {
                definition.push_str(&value.generate_definition(prefix, unions, true));
            }
            Self::Function { parameters } => {
                definition.push_str("fn(");
                for (i, parameter) in parameters.iter().enumerate() {
                    definition.push_str(&parameter.generate_definition(prefix, unions, true));
                    if i != parameters.len() - 1 {
                        definition.push_str(", ");
                    }
                }
                definition.push_str(") -> ()");
            }
            _ => unimplemented!(),
        }
        definition
    }

    fn is_table_or_tuple(&self) -> bool {
        match self {
            Self::Union {
                options,
                full_format: _,
            } => {
                let type_names: HashSet<_> = options.iter().map(Type::get_type_name).collect();
                type_names.contains("table") && type_names.contains("tuple")
            }
            _ => false,
        }
    }
}

impl LiteralValue {
    fn get_type_name(&self) -> String {
        match self {
            Self::String(string) => match string.as_str() {
                "=" => "EqualTo".to_owned(),
                ">" => "GreaterThan".to_owned(),
                "<" => "LesserThan".to_owned(),
                ">=" => "GreaterThanOrEqualTo".to_owned(),
                "<=" => "LesserThanOrEqualTo".to_owned(),
                "!=" => "NotEqualTo".to_owned(),
                "≥" | "≤" | "≠" => String::new(),
                _ => string.clone(),
            },
            Self::Boolean(boolean) => boolean.to_string().to_pascal_case(),
            _ => unreachable!(),
        }
    }
}

// TODO: add events to define
// TODO: cleanup
// TODO: adjust generated code with clippy?
// TODO: add tests
// TODO: improve docs
