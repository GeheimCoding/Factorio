use std::{fs, io};

use super::{ComplexType, Concept, Property, Prototype, PrototypeApiFormat, Type};

trait Generate {
    fn generate(&self, prefix: String, enum_variant: bool, unions: &mut Vec<String>) -> String;
}

trait StringTransformation {
    fn to_pascal_case(&self) -> String;
    fn to_rust_field_name(&self) -> String;
    fn to_rust_type(&self) -> String;
    fn to_optional_if(&self, optional: bool) -> String;
    fn to_doc_string(&self, indent: &str) -> String;
}

impl StringTransformation for String {
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
            if c == '_' || c == '.' || c == '-' || c == ':' {
                if let Some(next) = chars.next() {
                    pascal_case.push(next.to_ascii_uppercase());
                }
            } else {
                pascal_case.push(c);
            }
        }
        pascal_case
    }

    fn to_rust_field_name(&self) -> String {
        match self.as_str() {
            "type" => "type_".to_owned(),
            _ => self
                .replace('<', "")
                .replace('>', "")
                .replace(',', "")
                .replace('(', "")
                .replace(')', "")
                .replace(' ', "_"),
        }
    }

    fn to_rust_type(&self) -> String {
        match self.as_str() {
            "int8" => "i8".to_owned(),
            "int16" => "i16".to_owned(),
            "int32" => "i32".to_owned(),
            "float" => "f32".to_owned(),
            "double" => "f64".to_owned(),
            "string" => "String".to_owned(),
            "uint8" => "u8".to_owned(),
            "uint16" => "u16".to_owned(),
            "uint32" => "u32".to_owned(),
            "uint64" => "u64".to_owned(),
            _ => self.clone(),
        }
    }

    fn to_optional_if(&self, optional: bool) -> String {
        if optional {
            format!("Option<{self}>")
        } else {
            self.clone()
        }
    }

    fn to_doc_string(&self, indent: &str) -> String {
        self.replace('\n', &format!("\n{indent}/// "))
    }
}

fn generate_docs(
    description: Option<&String>,
    lists: Option<&Vec<String>>,
    examples: Option<&Vec<String>>,
    indent: bool,
) -> String {
    let mut result = String::new();
    let indent = if indent { "    " } else { "" };
    if let Some(description) = description {
        if !description.is_empty() {
            result.push_str(&format!(
                "{indent}/// {}\n",
                description.to_doc_string(indent)
            ));
        }
    }
    if let Some(lists) = lists {
        for list in lists {
            result.push_str(&format!("{indent}/// {}\n", list.to_doc_string(indent)));
        }
    }
    if let Some(examples) = examples {
        result.push_str(&format!(
            "{indent}///\n{indent}/// Examples:\n{indent}///\n"
        ));
        for example in examples {
            result.push_str(&format!("{indent}/// {}\n", example.to_doc_string(indent)));
        }
    }
    result
}

fn generate<G: Generate>(list: &[G]) -> String {
    let mut result = String::new();
    result.push_str(
        &list
            .iter()
            .map(|p| p.generate(String::new(), false, &mut vec![]))
            .collect::<Vec<_>>()
            .join("\n\n"),
    );
    result.push('\n');
    result
}

fn generate_struct(name: &str, parent: Option<&String>, properties: &Vec<Property>) -> String {
    let mut unions = vec![];
    let mut result = String::from(&format!("pub struct {} {{\n", name));
    if let Some(parent) = parent {
        result.push_str(&format!("    parent_: {parent},"));
        if !properties.is_empty() {
            result.push('\n');
        }
    }
    result.push_str(
        &properties
            .iter()
            .map(|p| p.generate(name.to_owned(), false, &mut unions))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    for union in unions {
        result.insert_str(0, &format!("{union}\n\n"));
    }
    result.push_str("\n}");
    result
}

fn generate_union(name: &str, options: &Vec<Type>, unions: &mut Vec<String>) -> String {
    let mut union = format!("pub enum {name} {{\n");
    for option in options {
        let result = option.generate(name.to_owned(), true, unions);
        let has_value = if let Type::Complex(complex) = option {
            if let ComplexType::Literal {
                value: _,
                description,
            } = complex.as_ref()
            {
                union.push_str(&generate_docs(description.as_ref(), None, None, true));
                false
            } else {
                true
            }
        } else {
            true
        };
        union.push_str(&format!(
            "    {}",
            result.to_rust_field_name().to_pascal_case()
        ));
        if has_value {
            union.push_str(&format!("({result})"));
        }
        union.push_str(",\n");
    }
    union.push('}');
    unions.push(union);
    name.to_owned()
}

impl PrototypeApiFormat {
    pub fn generate_prototype_api(
        &self,
        prototypes_path: &str,
        types_path: &str,
    ) -> io::Result<()> {
        fs::write(prototypes_path, generate(&self.prototypes))?;
        fs::write(types_path, generate(&self.types))
    }
}

impl Generate for Prototype {
    fn generate(&self, _prefix: String, _enum_variant: bool, _unions: &mut Vec<String>) -> String {
        // TODO: typename & custom_properties?
        format!(
            "{}{}",
            generate_docs(
                Some(&self.description),
                self.lists.as_ref(),
                self.examples.as_ref(),
                false,
            ),
            generate_struct(&self.name, self.parent.as_ref(), &self.properties)
        )
    }
}

impl Generate for Concept {
    fn generate(&self, prefix: String, enum_variant: bool, unions: &mut Vec<String>) -> String {
        let mut result = String::from(generate_docs(
            Some(&self.description),
            self.lists.as_ref(),
            self.examples.as_ref(),
            false,
        ));
        let is_new_type = match &self.type_ {
            Type::Simple(string) => string != "builtin",
            Type::Complex(complex) => match complex.as_ref() {
                ComplexType::Struct => {
                    result.push_str(&generate_struct(
                        &self.name,
                        self.parent.as_ref(),
                        self.properties
                            .as_ref()
                            .expect("there should be at least one property"),
                    ));
                    false
                }
                ComplexType::Union {
                    options,
                    full_format: _,
                } => {
                    generate_union(&self.name, options, unions);
                    false
                }
                _ => true,
            },
        };
        if is_new_type {}

        for union in unions {
            result.insert_str(0, &format!("{union}\n\n"));
        }
        result
    }
}

impl Generate for Property {
    fn generate(&self, prefix: String, enum_variant: bool, unions: &mut Vec<String>) -> String {
        // TODO: alt_name & override & default?
        format!(
            "{}    {}: {},",
            generate_docs(
                Some(&self.description),
                self.lists.as_ref(),
                self.examples.as_ref(),
                true
            ),
            self.name.to_rust_field_name(),
            self.type_
                .generate(
                    format!("{prefix}{}", self.name.to_pascal_case()),
                    enum_variant,
                    unions
                )
                .to_optional_if(self.optional)
        )
    }
}

impl Generate for Type {
    fn generate(&self, prefix: String, enum_variant: bool, unions: &mut Vec<String>) -> String {
        match self {
            Self::Simple(name) => name.to_rust_type(),
            Self::Complex(complex_type) => complex_type.generate(prefix, enum_variant, unions),
        }
    }
}

impl Generate for ComplexType {
    fn generate(&self, prefix: String, enum_variant: bool, unions: &mut Vec<String>) -> String {
        match self {
            Self::Array { value } => {
                format!("Vec<{}>", value.generate(prefix, enum_variant, unions))
            }
            // TODO: derive hash for key?
            Self::Dictionary { key, value } => format!(
                "HashMap<{}, {}>",
                key.generate(prefix.clone(), enum_variant, unions),
                value.generate(prefix, enum_variant, unions)
            ),
            Self::Tuple { values } => format!(
                "({})",
                values
                    .iter()
                    .map(|t| t.generate(prefix.clone(), enum_variant, unions))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Union {
                options,
                full_format: _,
            } => generate_union(&prefix, options, unions),
            Self::Literal {
                value,
                description: _,
            } => match value {
                super::ComplexTypeLiteralValueUnion::String(s) => {
                    if enum_variant {
                        s.to_pascal_case()
                    } else {
                        "String".to_owned()
                    }
                }
                super::ComplexTypeLiteralValueUnion::Number(_) => "f64".to_owned(),
                super::ComplexTypeLiteralValueUnion::Boolean(b) => {
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
            } => value.generate(prefix, enum_variant, unions),
            Self::Struct => prefix,
        }
    }
}
