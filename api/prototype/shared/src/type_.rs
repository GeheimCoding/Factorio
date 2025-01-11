use crate::format::{Context, DataType};
use crate::property::Property;
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
    pub fn generate(
        &self,
        prefix: &str,
        properties: Option<&Vec<Property>>,
        context: &Context,
    ) -> (String, Vec<String>) {
        match self {
            Type::Simple(simple) => simple.to_rust_type(context),
            Type::Complex(complex) => match complex.as_ref() {
                ComplexType::Array { value } => {
                    Self::generate_array(value, prefix, properties, context)
                }
                ComplexType::Dictionary { key, value } => {
                    Self::generate_dictionary(key, value, prefix, context)
                }
                ComplexType::Tuple { values } => Self::generate_tuple(values, prefix, context),
                ComplexType::Union {
                    options,
                    full_format,
                } => Self::generate_union(options, full_format, prefix, properties, context),
                ComplexType::Literal { value, description } => {
                    Self::generate_literal(value, description)
                }
                ComplexType::Type { value, description } => {
                    Self::generate_type(value, description, prefix, properties, context)
                }
                ComplexType::Struct => Self::generate_struct(prefix, properties, context),
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

    pub fn is_literal_value(&self) -> bool {
        match self {
            Type::Simple(_) => false,
            Type::Complex(complex) => match complex.as_ref() {
                ComplexType::Literal { .. } => true,
                ComplexType::Type { value, .. } => value.is_literal_value(),
                _ => false,
            },
        }
    }

    fn should_be_boxed(&self, context: &Context) -> bool {
        match self {
            Type::Simple(simple) => {
                matches!(context.context.get(simple), Some((_, DataType::Struct)))
            }
            Type::Complex(complex) => match complex.as_ref() {
                ComplexType::Type { value, .. } => value.should_be_boxed(context),
                _ => false,
            },
        }
    }

    fn postfix_variants(&self, prefix: &str) -> String {
        if self.is_union() && !prefix.ends_with("Variants") {
            format!("{prefix}Variants")
        } else {
            String::from(prefix)
        }
    }

    fn generate_array(
        value: &Type,
        prefix: &str,
        properties: Option<&Vec<Property>>,
        context: &Context,
    ) -> (String, Vec<String>) {
        let (inner, additional) =
            value.generate(&value.postfix_variants(prefix), properties, context);
        (format!("Vec<{inner}>"), additional)
    }

    fn generate_dictionary(
        key: &Type,
        value: &Type,
        prefix: &str,
        context: &Context,
    ) -> (String, Vec<String>) {
        let (inner_key, additional_key) = key.generate(prefix, None, context);
        let (inner_value, additional_value) = value.generate(prefix, None, context);
        let additional = [additional_key, additional_value].concat();
        (
            if inner_value == "true" {
                format!("std::collections::HashSet<{inner_key}>")
            } else {
                format!("std::collections::HashMap<{inner_key},{inner_value}>")
            },
            additional,
        )
    }

    fn generate_tuple(
        values: &Vec<Type>,
        prefix: &str,
        context: &Context,
    ) -> (String, Vec<String>) {
        let (inner, additional): (Vec<String>, Vec<Vec<String>>) = values
            .iter()
            .map(|value| value.generate(prefix, None, context))
            .unzip();
        let additional = additional
            .into_iter()
            .fold(vec![], |acc, e| [acc, e].concat());
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
        _full_format: &bool,
        prefix: &str,
        properties: Option<&Vec<Property>>,
        context: &Context,
    ) -> (String, Vec<String>) {
        let mut others = vec![];
        let mut union = format!("pub enum {prefix}{{");
        for option in options {
            let (inner, additional) =
                option.generate(&option.postfix_variants(prefix), properties, context);
            others.extend(additional);

            if option.is_literal_value() {
                union.push_str(&format!("{inner},"));
            } else if option.is_struct()
            /* // README: Adjustment [TODO] */
                && prefix != "LightDefinition"
            /* // README: Adjustment [TODO] */
            {
                union.push_str(&format!("{prefix}{inner},"));
            } else {
                let name = Self::remove_prefix(&inner).to_pascal_case();
                let inner = if option.should_be_boxed(context) {
                    format!("Box<{inner}>")
                } else {
                    inner
                };
                union.push_str(&format!("{name}({inner}),"));
            }
        }
        others.insert(0, format!("{union}}}"));
        (String::from(prefix), others)
    }

    fn remove_prefix(name: &str) -> String {
        if let Some(prefix) = name.rfind("::") {
            if let Some(start) = name.rfind("crate::") {
                Self::remove_prefix(&format!("{}{}", &name[..start], &name[prefix + 2..]))
            } else {
                String::from(name)
            }
        } else {
            String::from(name)
        }
    }

    fn generate_literal(
        value: &LiteralValue,
        _description: &Option<String>,
    ) -> (String, Vec<String>) {
        (value.generate(), vec![])
    }

    fn generate_type(
        value: &Type,
        _description: &String,
        prefix: &str,
        properties: Option<&Vec<Property>>,
        context: &Context,
    ) -> (String, Vec<String>) {
        value.generate(prefix, properties, context)
    }

    fn generate_struct(
        prefix: &str,
        properties: Option<&Vec<Property>>,
        context: &Context,
    ) -> (String, Vec<String>) {
        if let Some(properties) = properties {
            let mut others = Vec::new();
            let mut result = String::from("{");
            for property in properties {
                let (inner, additional) = property.generate(prefix, context);
                result.push_str(&format!("{inner},"));
                others.extend(additional);
            }
            // README: Adjustment [TODO]
            if prefix == "LightDefinition" {
                others.insert(0, format!("pub struct LightDefinitionStruct{result}}}"));
                return (String::from("LightDefinitionStruct"), others);
            }
            // README: Adjustment [TODO]
            (format!("{result}}}"), others)
        } else {
            unreachable!("struct {prefix} has no properties");
        }
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
