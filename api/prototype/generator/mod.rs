use std::{fs, io};

use super::{ComplexType, Property, Prototype, PrototypeApiFormat, Type};

trait Generate {
    fn generate(&self, prefix: String) -> String;
}

trait StringTransformation {
    fn to_pascal_case(&self) -> String;
    fn to_rust_field_name(&self) -> String;
    fn to_rust_type(&self) -> String;
    fn to_optional_if(&self, optional: bool) -> String;
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

    fn to_rust_field_name(&self) -> String {
        match self.as_str() {
            "type" => "type_".to_owned(),
            _ => self.clone(),
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
}

impl PrototypeApiFormat {
    pub fn generate_prototype_api(&self) -> io::Result<()> {
        fs::write("src/generated/prototypes.rs", self.generate(String::new()))
    }
}

impl Generate for PrototypeApiFormat {
    fn generate(&self, prefix: String) -> String {
        let mut result = String::new();
        result.push_str(
            &self
                .prototypes
                .iter()
                .map(|p| p.generate(prefix.clone()))
                .collect::<Vec<_>>()
                .join("\n\n"),
        );
        result.push('\n');
        result
    }
}

impl Generate for Prototype {
    fn generate(&self, prefix: String) -> String {
        // TODO: typename & custom_properties?
        let mut result = String::new();
        result.push_str(&format!("pub struct {} {{\n", self.name));
        if let Some(parent) = &self.parent {
            result.push_str(&format!("    parent_: {parent},"));
            if !self.properties.is_empty() {
                result.push('\n');
            }
        }
        result.push_str(
            &self
                .properties
                .iter()
                .map(|p| p.generate(format!("{prefix}{}", self.name.to_pascal_case())))
                .collect::<Vec<_>>()
                .join("\n"),
        );
        result.push_str("\n}");
        result
    }
}

impl Generate for Property {
    fn generate(&self, prefix: String) -> String {
        // TODO: alt_name & override & default?
        format!(
            "    {}: {},",
            self.name.to_rust_field_name(),
            self.type_
                .generate(format!("{prefix}{}", self.name.to_pascal_case()))
                .to_optional_if(self.optional)
        )
    }
}

impl Generate for Type {
    fn generate(&self, prefix: String) -> String {
        match self {
            Self::Simple(name) => name.to_rust_type(),
            Self::Complex(complex_type) => complex_type.generate(prefix),
        }
    }
}

impl Generate for ComplexType {
    fn generate(&self, prefix: String) -> String {
        match self {
            Self::Array { value } => format!("Vec<{}>", value.generate(prefix)),
            // TODO: derive hash for key?
            Self::Dictionary { key, value } => format!(
                "HashMap<{}, {}>",
                key.generate(prefix.clone()),
                value.generate(prefix)
            ),
            Self::Tuple { values } => format!(
                "({})",
                values
                    .iter()
                    .map(|t| t.generate(prefix.clone()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            _ => prefix,
        }
    }
}
