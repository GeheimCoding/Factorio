use std::{
    collections::HashSet,
    fmt::Display,
    fs::{self, File},
    io::{BufWriter, Write},
    path::PathBuf,
};

use serde::Deserialize;

trait PascalCase {
    fn to_pascal_case(&self) -> String;
}

impl PascalCase for String {
    fn to_pascal_case(&self) -> String {
        if self.is_empty() {
            return String::new();
        }
        let mut chars = self.chars();
        let mut pascal_case = String::from(chars.next().unwrap().to_ascii_uppercase());
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

trait GenerateDefinition {
    fn generate_definition(&self) -> String;
}

impl RuntimeApiFormat {
    pub fn generate_runtime_api(&self) -> std::io::Result<()> {
        let generated_path = PathBuf::from("src/generated");
        fs::create_dir_all(&generated_path)?;

        self.generate_classes(&generated_path)?;
        self.generate_events(&generated_path)?;
        self.generate_concepts(&generated_path)?;
        self.generate_defines(&generated_path)?;

        Ok(())
    }

    fn generate_definition<T: GenerateDefinition>(
        &self,
        file_path: PathBuf,
        definition_types: &Vec<T>,
    ) -> std::io::Result<()> {
        if file_path.exists() {
            fs::remove_file(&file_path)?;
        }
        let mut definition = String::new();
        for definition_type in definition_types {
            definition.push_str(&format!("{}\n", definition_type.generate_definition()));
        }
        definition.pop();
        fs::write(file_path, definition)?;

        Ok(())
    }

    fn generate_classes(&self, base_path: &PathBuf) -> std::io::Result<()> {
        self.generate_definition(base_path.join("classes.rs"), &self.classes)
    }

    fn generate_events(&self, base_path: &PathBuf) -> std::io::Result<()> {
        self.generate_definition(base_path.join("events.rs"), &self.events)
    }

    fn generate_concepts(&self, base_path: &PathBuf) -> std::io::Result<()> {
        self.generate_definition(base_path.join("concepts.rs"), &self.concepts)
    }

    fn generate_defines(&self, base_path: &PathBuf) -> std::io::Result<()> {
        self.generate_definition(base_path.join("defines.rs"), &self.defines)
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

impl GenerateDefinition for Class {
    fn generate_definition(&self) -> String {
        let mut definition = String::new();
        let mut inline_type_definition = String::new();
        let mut struct_definition = String::new();
        let mut unions = Vec::new();
        let prefix = &self.name;

        struct_definition.push_str(&format!("pub struct {} {{\n", prefix));
        for attribute in &self.attributes {
            let name = attribute.name.as_ref().unwrap();
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
            let typ = if attribute.optional {
                format!("Option<{typ}>")
            } else {
                typ.to_owned()
            };
            let name = if name == "type" {
                "typ"
            } else if name == "mod" {
                "mod_name"
            } else {
                name
            };
            struct_definition.push_str(&format!("    pub {}: {},\n", name, typ));
        }
        struct_definition.push_str("}\n");
        for union in unions {
            definition.push_str(&format!("{}\n\n", union));
        }
        definition.push_str(&inline_type_definition);
        definition.push_str(&struct_definition);
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

impl GenerateDefinition for Event {
    fn generate_definition(&self) -> String {
        let mut definition = String::new();
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
        self.generate_definition_internal("", false)
    }
}

impl Define {
    fn generate_definition_internal(&self, prefix: &str, is_sub: bool) -> String {
        // TODO: print description as doc
        let mut definition = String::new();
        let name = &format!("{}{}", prefix, self.name.to_pascal_case());

        // either we have variants
        if let Some(variants) = &self.values {
            if self.subkeys.is_some() {
                unreachable!();
            }
            definition.push_str(&format!("pub enum {} {{\n", name));
            for variant in variants {
                definition.push_str(&variant.generate_definition());
            }
            definition.push_str("}");
        // or sub-defines
        } else if let Some(sub_defines) = &self.subkeys {
            if self.values.is_some() {
                unreachable!();
            }
            for sub_define in sub_defines {
                definition.push_str(&format!(
                    "{}\n",
                    &sub_define.generate_definition_internal(name, true)
                ));
            }
            definition.pop();
        // or an empty struct
        } else {
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

impl GenerateDefinition for Concept {
    fn generate_definition(&self) -> String {
        let mut definition = String::new();
        let mut unions = Vec::new();
        let type_definition = self.typ.generate_definition(&self.name, &mut unions, false);
        for union in unions {
            definition.push_str(&format!("{}\n\n", &union));
        }
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
        format!("    {},\n", self.name.to_pascal_case())
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

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(name) => write!(f, "{}", Type::lua_type_to_rust_type(name)),
            Self::ComplexType(complex_type) => write!(f, "{complex_type}"),
        }
    }
}

impl Type {
    fn is_inline_type(&self) -> bool {
        match self {
            Self::String(_) => false,
            Self::ComplexType(complex_type) => complex_type.is_inline_type(),
        }
    }

    fn is_owned_type(&self) -> bool {
        if let Type::ComplexType(ComplexType::Literal { value, description }) = self {
            true
        } else {
            match self.get_type_name().as_str() {
                "table" | "nil" | "LuaObject" => true,
                _ => false,
            }
        }
    }

    fn lua_type_to_rust_type(type_name: &str) -> String {
        match type_name {
            "float" => "f32".to_owned(),
            "double" => "f64".to_owned(),
            "int" => "i32".to_owned(),
            "int8" => "i8".to_owned(),
            "uint" => "u32".to_owned(),
            "uint8" => "u8".to_owned(),
            "uint16" => "u16".to_owned(),
            "uint64" => "u64".to_owned(),
            "number" => "f64".to_owned(),
            "string" => "String".to_owned(),
            "boolean" => "bool".to_owned(),
            name if name.starts_with("defines.") => name[8..].to_owned().to_pascal_case(),
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

impl Display for ComplexType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Type { value, description } => write!(f, "{value}"),
            Self::Union {
                options,
                full_format,
            } => write!(
                f,
                "Union of [{}]", // TODO: implement enum for each variant?
                options
                    .iter()
                    .map(Type::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Array { value } => write!(f, "Vec<{value}>"),
            Self::Dictionary { key, value } | Self::LuaCustomTable { key, value } => {
                write!(f, "HashMap<{key}, {value}>")
            }
            Self::Function { parameters } => todo!(),
            Self::Literal { value, description } => write!(f, "{value:?}"),
            Self::LuaLazyLoadedValue { value } => write!(f, "{value}"),
            Self::Struct { attributes } => todo!(),
            Self::Table {
                parameters,
                variant_parameter_groups,
                variant_parameter_description,
            }
            | Self::Tuple {
                parameters,
                variant_parameter_groups,
                variant_parameter_description,
            } => todo!(),
        }
    }
}

impl ComplexType {
    fn is_inline_type(&self) -> bool {
        match self {
            Self::Table {
                parameters,
                variant_parameter_groups,
                variant_parameter_description,
            }
            | Self::Tuple {
                parameters,
                variant_parameter_groups,
                variant_parameter_description,
            } => true,
            _ => false,
        }
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
        let name = self.name.as_ref().unwrap();
        let prefix = &format!("{}{}", prefix, name.to_pascal_case());
        let typ = Type::lua_type_to_rust_type(&self.typ.generate_definition(prefix, unions, true));
        let typ = if self.optional {
            format!("Option<{}>", typ)
        } else {
            typ
        };
        let name = if name == "type" {
            "typ".to_owned()
        } else if name == "mod" {
            "mod_name".to_owned()
        } else if name == "noisePersistence" {
            "noise_persistence".to_owned()
        } else if name == "_" {
            format!("field_{}", self.order)
        } else {
            name.replace("-", "_")
        };
        definition.push_str(&format!("    pub {}: {},\n", name, typ));
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

impl Type {
    fn get_type_name(&self) -> String {
        match self {
            Self::String(string) => string.clone(),
            Self::ComplexType(complex_type) => complex_type.get_type_name(),
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
                let mut definition = String::new();
                if !is_nested {
                    definition.push_str(&format!("type {prefix} = "));
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
            Self::Type { value, description } => value.get_type_name(),
            Self::Literal { value, description } => value.get_type_name(),
            Self::Table {
                parameters,
                variant_parameter_groups,
                variant_parameter_description,
            } => "table".to_string(),
            Self::Tuple {
                parameters,
                variant_parameter_groups,
                variant_parameter_description,
            } => "tuple".to_string(),
            Self::Array { value } => "array".to_string(),
            Self::Dictionary { key, value } => "dictionary".to_owned(),
            _ => unimplemented!(),
        }
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
                full_format,
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
                union_definition.push_str(&format!("pub enum {} {{\n", prefix));
                let array_count = options
                    .iter()
                    .map(Type::get_type_name)
                    .filter(|name| name == "array")
                    .count();
                for option in options {
                    let mut type_name = option.get_type_name();
                    if option.is_owned_type() {
                        if !type_name.is_empty() {
                            union_definition
                                .push_str(&format!("    {},\n", type_name.to_pascal_case()));
                        }
                    } else {
                        let typ = if type_name == "array" {
                            let array_definition =
                                option.generate_definition(&prefix, unions, true);
                            if array_count > 1 {
                                type_name.push('_');
                                type_name
                                    .push_str(&array_definition[4..array_definition.len() - 1]);
                            }
                            array_definition
                        } else {
                            Type::lua_type_to_rust_type(&type_name)
                        };
                        union_definition.push_str(&format!(
                            "    {}({}),\n",
                            type_name.to_pascal_case(),
                            typ
                        ));
                    }
                }
                union_definition.push_str("}");

                if is_nested {
                    unions.push(format!("{union_definition}"));
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
                table_definition.push_str(&format!("pub struct {} {{\n", prefix));
                for parameter in parameters {
                    table_definition.push_str(&parameter.get_definition(unions, prefix));
                }
                if let Some(variant_parameter_groups) = variant_parameter_groups {
                    table_definition.push_str(&format!(
                        "    pub attributes: Option<{}Attributes>,\n",
                        prefix
                    ));
                    let mut variant_definition = String::new();
                    let prefix = &format!("{}Attributes", prefix);
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
                        definition.push_str(&format!("pub struct {} {{\n", name));
                        for parameter in &variant_parameter_group.parameters {
                            let name = parameter.name.as_ref().unwrap().replace("-", "_");
                            let prefix = &format!("{}{}", prefix, name.to_pascal_case());
                            let typ = Type::lua_type_to_rust_type(
                                &parameter.typ.generate_definition(prefix, unions, true),
                            );
                            let typ = if parameter.optional {
                                format!("Option<{typ}>")
                            } else {
                                typ
                            };
                            let name = if name == "type" { "typ" } else { &name };
                            let name = if name == "mod" { "mod_name" } else { &name };
                            definition.push_str(&format!("    pub {}: {},\n", name, typ));
                        }
                        definition.push_str("}\n\n");
                    }
                    variant_definition.push_str("}\n\n");
                    definition.push_str(&variant_definition);
                }
                table_definition.push_str("}");
                definition.push_str(&table_definition);
            }
            Self::Array { value } => {
                if !is_nested {
                    definition.push_str(&format!("type {prefix} = "));
                }
                definition.push_str(&format!(
                    "Vec<{}>",
                    value.generate_definition(prefix, unions, true)
                ));
                if !is_nested {
                    definition.push(';');
                }
            }
            Self::Dictionary { key, value } | Self::LuaCustomTable { key, value } => {
                if !is_nested {
                    definition.push_str(&format!("type {prefix} = "));
                }
                let mut is_map = true;
                if let Type::ComplexType(ComplexType::Literal { value, description }) =
                    value.as_ref()
                {
                    if value.get_type_name() == "True" {
                        definition.push_str(&format!(
                            "HashSet<{}>",
                            key.generate_definition(prefix, unions, true)
                        ));
                        is_map = false;
                    }
                }
                if is_map {
                    definition.push_str(&format!(
                        "HashMap<{}, {}>",
                        key.generate_definition(prefix, unions, true),
                        value.generate_definition(prefix, unions, true)
                    ));
                }
                if !is_nested {
                    definition.push(';');
                }
            }
            Self::Literal { value, description } => {
                definition.push_str(&value.get_type_name());
            }
            Self::Struct { attributes } => {
                definition.push_str(&format!("pub struct {} {{\n", prefix));
                for attribute in attributes {
                    let name = attribute.name.as_ref().unwrap();
                    let prefix = &format!("{}{}", prefix, name.to_pascal_case());
                    let typ = Type::lua_type_to_rust_type(
                        &attribute.typ.generate_definition(prefix, unions, true),
                    );
                    definition.push_str(&format!("    pub {}: {},\n", name, typ));
                }
                definition.push_str("}");
            }
            Self::LuaLazyLoadedValue { value } => {
                definition.push_str(&value.generate_definition(prefix, unions, true));
            }
            _ => unimplemented!(),
        }
        definition
    }

    fn is_table_or_tuple(&self) -> bool {
        match self {
            Self::Union {
                options,
                full_format,
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

// TODO: handle methods from class
// TODO: fix dictionary in union
// TODO: fix defines.types (add Events type?)
// TODO: model base class better? (e.g. for filter types)
// TODO: remove Union postfix for named types
// TODO: add descriptions
// TODO: fix clippy lints
// TODO: add tests
// TODO: cleanup
