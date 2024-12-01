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
    pub fn generate(&self, prefix: &str) -> (String, Vec<String>) {
        match self {
            Type::Simple(simple) => (simple.to_rust_type(), vec![]),
            Type::Complex(complex) => match complex.as_ref() {
                ComplexType::Array { value } => Self::generate_array(value, prefix),
                ComplexType::Dictionary { key, value } => {
                    Self::generate_dictionary(key, value, prefix)
                }
                ComplexType::Tuple { values } => Self::generate_tuple(values, prefix),
                ComplexType::Union {
                    options,
                    full_format,
                } => Self::generate_union(options, full_format, prefix),
                ComplexType::Literal { value, description } => {
                    Self::generate_literal(value, description)
                }
                ComplexType::Type { value, description } => {
                    Self::generate_type(value, description, prefix)
                }
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

    fn generate_array(value: &Type, prefix: &str) -> (String, Vec<String>) {
        let (inner, additional) = value.generate(prefix);
        (format!("Vec<{inner}>"), additional)
    }

    fn generate_dictionary(key: &Type, value: &Type, prefix: &str) -> (String, Vec<String>) {
        let (inner_key, additional_key) = key.generate(prefix);
        let (inner_value, additional_value) = value.generate(prefix);
        let additional = [additional_key, additional_value].concat();
        (
            format!("std::collections::HashMap<{inner_key},{inner_value}>"),
            additional,
        )
    }

    fn generate_tuple(values: &Vec<Type>, prefix: &str) -> (String, Vec<String>) {
        let (inner, additional): (Vec<String>, Vec<Vec<String>>) =
            values.iter().map(|value| value.generate(prefix)).unzip();
        let additional = additional
            .into_iter()
            .fold(vec![], |mut acc, e| [acc, e].concat());
        (
            format!(
                "({})",
                inner.into_iter().map(|v| v).collect::<Vec<_>>().join(",")
            ),
            additional,
        )
    }

    fn generate_union(
        options: &Vec<Type>,
        full_format: &bool,
        prefix: &str,
    ) -> (String, Vec<String>) {
        let name = format!("{prefix}Variants");
        let mut others = vec![];
        let mut union = format!("pub enum {name}{{");
        for option in options {
            let (inner, additional) = option.generate(prefix);
            union.push_str(&format!("{inner},"));
            others.extend(additional);
        }
        others.push(format!("{union}}}"));
        (name, others)
    }

    fn generate_literal(
        value: &LiteralValue,
        description: &Option<String>,
    ) -> (String, Vec<String>) {
        (value.generate(), vec![])
    }

    fn generate_type(value: &Type, description: &String, prefix: &str) -> (String, Vec<String>) {
        value.generate(prefix)
    }
}

impl LiteralValue {
    pub fn generate(&self) -> String {
        match self {
            LiteralValue::String(s) => format!("{}", s.to_pascal_case()),
            LiteralValue::Number(n) => format!("{n}"),
            LiteralValue::Bool(b) => format!("{b}"),
        }
    }
}
