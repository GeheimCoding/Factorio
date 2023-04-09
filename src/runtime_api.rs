use std::fmt::Display;

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
            if c == '_' {
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
pub struct RuntimeApi {
    /// The application this documentation is for. Will always be "factorio".
    pub application: String,
    /// Indicates the stage this documentation is for. Will always be "runtime" (as opposed to "data"; see the Data Lifecycle for more detail).
    pub stage: String,
    /// The version of the game that this documentation is for. An example would be "1.1.30".
    pub application_version: String,
    /// The version of the machine-readable format itself. It is incremented every time the format changes. The version this documentation reflects is stated at the top.
    pub api_version: u8,
    /// The list of classes (LuaObjects) the API provides. Equivalent to the classes page.
    pub classes: Vec<Class>,
    /// The list of events that the API provides. Equivalent to the events page.
    pub events: Vec<Event>,
    /// The list of defines that the game uses. Equivalent to the defines page.
    pub defines: Vec<Define>,
    /// The list of types that are built into Lua itself. Equivalent to the built-in types page.
    pub builtin_types: Vec<BuiltinType>,
    /// The list of concepts of various types that the API uses. Equivalent to the concepts page.
    pub concepts: Vec<Concept>,
    /// The list of objects that the game provides as global variables to serve as entry points to the API.
    pub global_objects: Vec<GlobalObject>,
    /// The list of functions that the game provides as global variables to provide some specific functionality.
    pub global_functions: Vec<Method>,
}

// Top level types

#[derive(Debug, Deserialize)]
pub struct Class {
    /// The name of the class.
    pub name: String,
    /// The order of the class as shown in the html.
    pub order: u8,
    ///  The text description of the class.
    pub description: String,
    /// A list of strings containing additional information about the class.
    pub notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    pub examples: Option<Vec<String>>,
    /// The methods that are part of the class.
    pub methods: Vec<Method>,
    /// The attributes that are part of the class.
    pub attributes: Vec<Attribute>,
    /// A list of operators on the class. They are called call, index, or length and have the format of either a Method or an Attribute.
    pub operators: Vec<Operator>,
    /// Whether the class is never itself instantiated, only inherited from.
    #[serde(rename = "abstract")]
    pub abstract_flag: bool,
    /// A list of the names of the classes that his class inherits from.
    pub base_classes: Option<Vec<String>>,
}

impl Display for Class {
    // TODO: add optional attributes
    // TODO: resolve defines. types
    // TODO: solve inheritance
    // TODO: add descriptions as doc
    // TODO: order elements by order?
    // TODO: add derives?
    // TODO: refactor
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut inline_types = Vec::new();
        writeln!(f, "pub struct {} {{", self.name)?;
        for attribute in &self.attributes {
            // if !attribute.description.is_empty() {
            //     writeln!(f, "    /// {}", attribute.description)?;
            // }
            let typ = &attribute.typ;
            let name = attribute.name.as_ref().unwrap();
            let typ = if typ.is_inline_type() {
                inline_types.push(typ);
                format!("{}{}", self.name, name.to_pascal_case())
            } else {
                typ.to_string()
            };
            let typ = if attribute.optional {
                format!("Option<{typ}>")
            } else {
                typ
            };
            let name = if name == "type" { "typ" } else { name };
            writeln!(f, "    pub {}: {},", name, typ)?;
        }
        writeln!(f, "}}")?;

        // TODO: print inline types

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "name")]
pub enum Operator {
    Call(Method),
    Index(Attribute),
    Length(Attribute),
}

#[derive(Debug, Deserialize)]
pub struct Event {
    /// The name of the event.
    pub name: String,
    /// The order of the event as shown in the html.
    pub order: u8,
    /// The text description of the event.
    pub description: String,
    /// A list of strings containing additional information about the event.
    pub notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    pub examples: Option<Vec<String>>,
    /// The event-specific information that is provided.
    pub data: Vec<Parameter>,
}

/// Defines can be recursive in nature, meaning one Define can have multiple sub-Defines that have the same structure. These are singled out as subkeys instead of values.
#[derive(Debug, Deserialize)]
pub struct Define {
    /// The name of the define.
    pub name: String,
    /// The order of the define as shown in the html.
    pub order: u8,
    /// The text description of the define.
    pub description: String,
    /// The members of the define.
    pub values: Option<Vec<BasicMember>>,
    /// A list of sub-defines.
    pub subkeys: Option<Vec<Define>>,
}

#[derive(Debug, Deserialize)]
pub struct BuiltinType {
    /// The name of the builtin type.
    pub name: String,
    /// The order of the builtin type as shown in the html.
    pub order: u8,
    /// The text description of the builtin type.
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Concept {
    /// The name of the concept.
    pub name: String,
    /// The order of the concept as shown in the html.
    pub order: u8,
    /// The text description of the concept.
    pub description: String,
    /// A list of strings containing additional information about the concept.
    pub notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    pub examples: Option<Vec<String>>,
    /// The type of the concept.
    #[serde(rename = "type")]
    pub typ: Type,
}

#[derive(Debug, Deserialize)]
pub struct GlobalObject {
    /// The global variable name of the object.
    pub name: String,
    /// The order of the global object as shown in the html.
    pub order: u8,
    /// The text description of the global object.
    pub description: String,
    /// The class name of the global object.
    #[serde(rename = "type")]
    pub typ: String,
}

// Common structures

#[derive(Debug, Deserialize)]
pub struct BasicMember {
    /// The name of the member.
    pub name: String,
    /// The order of the member as shown in the html.
    pub order: u8,
    /// The text description of the member.
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct EventRaised {
    /// The name of the event being raised.
    pub name: String,
    /// The order of the member as shown in the html.
    pub order: u8,
    /// The text description of the raised event.
    pub description: String,
    /// The timeframe during which the event is raised. One of "instantly", "current_tick", or "future_tick".
    pub timeframe: String,
    /// Whether the event is always raised, or only dependant on a certain condition.
    pub optional: bool,
}

/// A type is either a string, in which case that string is the simple type. Otherwise, a type is a table:
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Type {
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

    fn lua_type_to_rust_type(type_name: &str) -> &str {
        match type_name {
            "float" => "f32",
            "double" => "f64",
            "int" => "i32",
            "int8" => "i8",
            "uint" => "u32",
            "uint8" => "u8",
            "uint16" => "u16",
            "uint64" => "u64",
            "number" => "f64",
            "string" => "String",
            "boolean" => "bool",
            name => name,
        }
    }
}

/// Depending on complex_type, there are additional members:
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "complex_type")]
pub enum ComplexType {
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
pub enum LiteralValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    /// The name of the parameter.
    pub name: Option<String>,
    /// The order of the parameter as shown in the html.
    pub order: u8,
    /// The text description of the parameter.
    pub description: String,
    /// The type of the parameter.
    #[serde(rename = "type")]
    pub typ: Type,
    /// Whether the type is optional or not.
    pub optional: bool,
}

#[derive(Debug, Deserialize)]
pub struct ParameterGroup {
    /// The name of the parameter group.
    pub name: String,
    /// The order of the parameter group as shown in the html.
    pub order: u8,
    /// The text description of the parameter group.
    pub description: Option<String>,
    /// The parameters that the group adds.
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Deserialize)]
pub struct Method {
    /// The name of the method.
    pub name: Option<String>,
    /// The order of the method as shown in the html.
    pub order: u8,
    /// The text description of the method.
    pub description: String,
    /// A list of strings containing additional information about the method.
    pub notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    pub examples: Option<Vec<String>>,
    /// A list of events that this method might raise when called.
    pub raises: Option<Vec<EventRaised>>,
    /// A list of strings specifying the sub-type (of the class) that the method applies to.
    pub subclasses: Option<Vec<String>>,
    /// The parameters of the method. How to interpret them depends on the takes_table member.
    pub parameters: Vec<Parameter>,
    /// The optional parameters that depend on one of the main parameters. Only applies if takes_table is true.
    pub variant_parameter_groups: Option<Vec<ParameterGroup>>,
    /// The text description of the optional parameter groups.
    pub variant_parameter_description: Option<String>,
    /// The type of the variadic arguments of the method, if it accepts any.
    pub variadic_type: Option<Type>,
    /// The description of the variadic arguments of the method, if it accepts any.
    pub variadic_description: Option<String>,
    /// Whether the method takes a single table with named parameters or a sequence of unnamed parameters.
    pub takes_table: bool,
    /// If takes_table is true, whether that whole table is optional or not.
    pub table_is_optional: Option<bool>,
    /// The return values of this method, which can contain zero, one, or multiple values. Note that these have the same structure as parameters, but do not specify a name.
    pub return_values: Vec<Parameter>,
}

#[derive(Debug, Deserialize)]
pub struct Attribute {
    /// The name of the attribute.
    pub name: Option<String>,
    /// The order of the attribute as shown in the html.
    pub order: u8,
    /// The text description of the attribute.
    pub description: String,
    /// A list of strings containing additional information about the attribute.
    pub notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    pub examples: Option<Vec<String>>,
    /// A list of events that this attribute might raise when written to.
    pub raises: Option<Vec<EventRaised>>,
    /// A list of strings specifying the sub-type (of the class) that the attribute applies to.
    pub subclasses: Option<Vec<String>>,
    /// The type of the attribute.
    #[serde(rename = "type")]
    pub typ: Type,
    /// Whether the attribute is optional or not.
    pub optional: bool,
    /// Whether the attribute can be read from.
    pub read: bool,
    /// Whether the attribute can be written to.
    pub write: bool,
}