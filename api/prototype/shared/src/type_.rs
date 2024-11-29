use crate::transformation::Transformation;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Type {
    Simple(String),
    Complex(Box<ComplexType>),
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "complex_type")]
#[serde(rename_all = "snake_case")]
pub enum ComplexType {
    Array {
        value: Type,
    },
    Dictionary {
        key: Type,
        value: Type,
    },
    Tuple {
        values: Vec<Type>,
    },
    Union {
        options: Vec<Type>,
        full_format: bool,
    },
    Literal {
        value: LiteralValue,
        description: Option<String>,
    },
    Type {
        value: Type,
        description: String,
    },
    Struct,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum LiteralValue {
    String(String),
    Number(f64),
    Bool(bool),
}

impl Type {
    pub fn generate(&self) -> String {
        match self {
            Type::Simple(simple) => simple.to_rust_type(),
            Type::Complex(complex) => match complex.as_ref() {
                ComplexType::Array { value } => Self::generate_array(value),
                ComplexType::Dictionary { key, value } => Self::generate_dictionary(key, value),
                ComplexType::Tuple { values } => Self::generate_tuple(values),
                ComplexType::Union {
                    options,
                    full_format,
                } => Self::generate_union(options, full_format),
                ComplexType::Literal { value, description } => {
                    Self::generate_literal(value, description)
                }
                ComplexType::Type { value, description } => Self::generate_type(value, description),
                ComplexType::Struct => unimplemented!("nothing to generate"),
            },
        }
    }

    pub fn is_struct(&self) -> bool {
        match self {
            Type::Simple(_) => false,
            Type::Complex(complex) => match complex.as_ref() {
                ComplexType::Struct => true,
                ComplexType::Type { value, .. } => value.is_struct(),
                _ => false,
            },
        }
    }

    pub fn is_union(&self) -> bool {
        match self {
            Type::Simple(_) => false,
            Type::Complex(complex) => match complex.as_ref() {
                ComplexType::Union { .. } => true,
                ComplexType::Type { value, .. } => value.is_union(),
                _ => false,
            },
        }
    }

    pub fn contains_struct(&self) -> bool {
        match self {
            Type::Simple(_) => false,
            Type::Complex(complex) => match complex.as_ref() {
                ComplexType::Union { options, .. } => options.iter().any(|o| o.is_struct()),
                ComplexType::Struct => true,
                _ => false,
            },
        }
    }

    fn generate_array(value: &Type) -> String {
        format!("Vec<{}>", value.generate())
    }

    fn generate_dictionary(key: &Type, value: &Type) -> String {
        format!(
            "std::collections::HashMap<{},{}>",
            key.generate(),
            value.generate()
        )
    }

    fn generate_tuple(values: &Vec<Type>) -> String {
        format!(
            "({})",
            values
                .iter()
                .map(|v| v.generate())
                .collect::<Vec<_>>()
                .join(",")
        )
    }

    fn generate_union(options: &Vec<Type>, full_format: &bool) -> String {
        // TODO: only return name of union (needs prefix?)
        String::from("todo!()")
    }

    fn generate_literal(value: &LiteralValue, description: &Option<String>) -> String {
        String::from("todo!()")
    }

    fn generate_type(value: &Type, description: &String) -> String {
        value.generate()
    }
}
