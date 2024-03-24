#![allow(unused)]
use std::collections::HashSet;

use serde::Deserialize;

use crate::generator::{generate_union, Generate, Macro, StringTransformation};

use super::runtime::{
    attribute::Attribute,
    parameter::{Parameter, ParameterGroup},
};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Type {
    Simple(String),
    Complex(Box<ComplexType>),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "complex_type")]
#[serde(rename_all = "snake_case")]
pub enum ComplexType {
    Array {
        /// The type of the elements of the array.
        value: Type,
    },
    Dictionary {
        /// The type of the keys of the dictionary.
        key: Type,
        /// The type of the values of the dictionary.
        value: Type,
    },
    Tuple(Tuple),
    Union {
        /// A list of all compatible types for this type.
        options: Vec<Type>,
        /// Whether the options of this union have a description or not.
        full_format: bool,
    },
    Literal {
        /// The value of the literal.
        value: ComplexTypeLiteralValueUnion,
        /// The text description of the literal, if any.
        description: Option<String>,
    },
    Type {
        /// The actual type. This format for types is used when they have descriptions attached to them.
        value: Type,
        /// The text description of the type.
        description: String,
    },
    /// Special type with no additional members. The properties themselves are listed on the API member that uses this type.
    Struct,
    #[serde(rename = "LuaCustomTable")]
    LuaCustomTable {
        /// The type of the keys of the LuaCustomTable.
        key: Type,
        /// The type of the values of the LuaCustomTable.
        value: Type,
    },
    Function {
        /// The types of the function arguments.
        parameters: Vec<Type>,
    },
    #[serde(rename = "LuaLazyLoadedValue")]
    LuaLazyLoadedValue {
        /// The type of the LuaLazyLoadedValue.
        value: Type,
    },
    #[serde(rename = "LuaStruct")]
    LuaStruct {
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
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Tuple {
    Tuple {
        /// The types of the members of this tuple in order.
        values: Vec<Type>,
    },
    Table {
        /// The parameters present in the table.
        parameters: Vec<Parameter>,
        /// The optional parameters that depend on one of the main parameters.
        variant_parameter_groups: Option<Vec<ParameterGroup>>,
        /// The text description of the optional parameter groups.
        variant_parameter_description: Option<String>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ComplexTypeLiteralValueUnion {
    String(String),
    Number(f64),
    Boolean(bool),
}

impl Generate for Type {
    fn generate(
        &self,
        prefix: String,
        enum_variant: bool,
        indent: usize,
        unions: &mut Vec<String>,
        class_names: &HashSet<String>,
    ) -> String {
        match self {
            Self::Simple(name) => name.to_rust_type(class_names),
            Self::Complex(complex_type) => {
                complex_type.generate(prefix, enum_variant, indent, unions, class_names)
            }
        }
    }
}

impl Generate for ComplexType {
    fn generate(
        &self,
        prefix: String,
        enum_variant: bool,
        indent: usize,
        unions: &mut Vec<String>,
        class_names: &HashSet<String>,
    ) -> String {
        match self {
            Self::Array { value } => {
                format!(
                    "Vec<{}>",
                    value.generate(prefix, enum_variant, indent, unions, class_names)
                )
            }
            // TODO: derive hash for key?
            Self::Dictionary { key, value } | Self::LuaCustomTable { key, value } => format!(
                "HashMap<{}, {}>",
                key.generate(prefix.clone(), enum_variant, indent, unions, class_names),
                value.generate(prefix, enum_variant, indent, unions, class_names)
            ),
            Self::Tuple(Tuple::Tuple { values }) => {
                let tuple = format!(
                    "{}",
                    values
                        .iter()
                        .map(|t| t.generate(
                            prefix.clone(),
                            enum_variant,
                            indent,
                            unions,
                            class_names
                        ))
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                if values.len() > 1 {
                    format!("({tuple})")
                } else {
                    tuple
                }
            }
            Self::Union {
                options,
                full_format,
            } => generate_union(&prefix, options, unions, None, class_names),
            Self::Literal { value, description } => match value {
                ComplexTypeLiteralValueUnion::String(s) => {
                    if enum_variant && s != "item" {
                        s.to_pascal_case()
                    } else {
                        "String".to_owned()
                    }
                }
                ComplexTypeLiteralValueUnion::Number(_) => "f64".to_owned(),
                ComplexTypeLiteralValueUnion::Boolean(b) => {
                    if enum_variant {
                        b.to_string()
                    } else {
                        "bool".to_owned()
                    }
                }
            },
            Self::Type { value, description } => {
                value.generate(prefix, enum_variant, indent, unions, class_names)
            }
            Self::Struct => prefix,
            Self::Function { parameters } => todo!(),
            Self::LuaLazyLoadedValue { value } => {
                value.generate(prefix, enum_variant, indent, unions, class_names)
            }
            Self::LuaStruct { attributes } => {
                format!(
                    "{}\npub struct {prefix} {{\n{}\n}}",
                    Macro::DebugDeserialize.to_string(),
                    attributes
                        .iter()
                        .map(|a| a.generate(
                            prefix.clone(),
                            enum_variant,
                            indent + 1,
                            unions,
                            class_names
                        ))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
            Self::Table {
                parameters,
                variant_parameter_groups,
                variant_parameter_description,
            }
            | Self::Tuple(Tuple::Table {
                parameters,
                variant_parameter_groups,
                variant_parameter_description,
            }) => {
                let mut result = format!(
                    "{}\npub struct {prefix} {{\n{}\n",
                    Macro::DebugDeserialize.to_string(),
                    parameters
                        .iter()
                        .map(|p| p.generate(
                            prefix.clone(),
                            enum_variant,
                            indent + 1,
                            unions,
                            class_names
                        ))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
                if let Some(description) = variant_parameter_description {
                    result.push_str(&format!("    /// {description}\n"));
                }
                if let Some(groups) = variant_parameter_groups {
                    let prefix = format!("{prefix}Attributes");
                    result.push_str(&format!("    pub attributes: Option<{prefix}>"));
                    let mut union = format!(
                        "{}{}pub enum {prefix} {{\n",
                        Macro::DebugDeserializeEnumAsInner.to_string(),
                        Macro::SerdeUntagged.to_string(),
                    );
                    for group in groups {
                        let name = group.name().to_pascal_case();
                        let prefix = format!("{prefix}{name}");
                        union.push_str(&format!("    {}({}),\n", name, prefix));
                        let group =
                            group.generate(prefix, enum_variant, indent, unions, class_names);
                        unions.push(group);
                    }
                    union.push('}');
                    unions.insert(0, union);
                }
                result.push('}');
                result
            }
        }
    }
}
