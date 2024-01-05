#![allow(unused)]
use serde::Deserialize;

use crate::generator::{generate_union, Generate, StringTransformation};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Type {
    Simple(String),
    Complex(Box<ComplexType>),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "complex_type")]
#[serde(rename_all = "camelCase")]
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
    Tuple {
        /// The types of the members of this tuple in order.
        values: Vec<Type>,
    },
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
    ) -> String {
        match self {
            Self::Simple(name) => name.to_rust_type(),
            Self::Complex(complex_type) => {
                complex_type.generate(prefix, enum_variant, indent, unions)
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
    ) -> String {
        match self {
            Self::Array { value } => {
                format!(
                    "Vec<{}>",
                    value.generate(prefix, enum_variant, indent, unions)
                )
            }
            // TODO: derive hash for key?
            Self::Dictionary { key, value } => format!(
                "HashMap<{}, {}>",
                key.generate(prefix.clone(), enum_variant, indent, unions),
                value.generate(prefix, enum_variant, indent, unions)
            ),
            Self::Tuple { values } => format!(
                "({})",
                values
                    .iter()
                    .map(|t| t.generate(prefix.clone(), enum_variant, indent, unions))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Union {
                options,
                full_format: _,
            } => generate_union(&prefix, options, unions, None),
            Self::Literal {
                value,
                description: _,
            } => match value {
                ComplexTypeLiteralValueUnion::String(s) => {
                    if enum_variant {
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
            Self::Type {
                value,
                description: _,
            } => value.generate(prefix, enum_variant, indent, unions),
            Self::Struct => prefix,
        }
    }
}
