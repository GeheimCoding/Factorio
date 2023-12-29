use std::{fs, io};

use super::{Property, Prototype, PrototypeApiFormat, Type};

trait Generate {
    fn generate(&self, prefix: String) -> String;
}

trait StringTransformation {
    fn to_pascal_case(&self) -> String;
    fn to_rust_field_name(&self) -> String;
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
            result.push_str(&format!("    parent_: {parent},\n"));
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
        )
    }
}

impl Generate for Type {
    fn generate(&self, prefix: String) -> String {
        prefix
    }
}
