use std::{
    collections::{HashMap, HashSet},
    fs, io,
};

use self::{
    prototype::{api_format::PrototypeApiFormat, property::Property},
    runtime::api_format::RuntimeApiFormat,
    type_::{ComplexType, Type},
};

pub mod prototype;
pub mod runtime;
mod type_;

trait Generate {
    fn generate(
        &self,
        prefix: String,
        enum_variant: bool,
        indent: usize,
        unions: &mut Vec<String>,
    ) -> String;
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
            "mod" => "mod_".to_owned(),
            _ => self
                .replace('<', "")
                .replace('>', "")
                .replace(',', "")
                .replace('(', "")
                .replace(')', "")
                .replace(' ', "_")
                .replace('-', "_")
                .replace('=', "_"),
        }
    }

    fn to_rust_type(&self) -> String {
        match self.as_str() {
            "int8" => "i8".to_owned(),
            "int16" => "i16".to_owned(),
            "int32" | "int" => "i32".to_owned(),
            "float" => "f32".to_owned(),
            "double" => "f64".to_owned(),
            "string" => "String".to_owned(),
            "uint8" => "u8".to_owned(),
            "uint16" => "u16".to_owned(),
            "uint32" | "uint" => "u32".to_owned(),
            "uint64" => "u64".to_owned(),
            "boolean" => "bool".to_owned(),
            s => {
                if s.starts_with("defines.") {
                    let parts = s.split("defines.").collect::<Vec<_>>();
                    parts[1].to_owned().to_pascal_case()
                } else {
                    self.clone()
                }
            }
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
    notes: Option<&Vec<String>>,
    examples: Option<&Vec<String>>,
    indent: usize,
) -> String {
    let mut result = String::new();
    let indent = "    ".repeat(indent);
    if let Some(description) = description {
        if !description.is_empty() {
            result.push_str(&format!(
                "{indent}/// {}\n",
                description.to_doc_string(&indent)
            ));
        }
    }
    if let Some(lists) = lists {
        for list in lists {
            result.push_str(&format!("{indent}/// {}\n", list.to_doc_string(&indent)));
        }
    }
    if let Some(notes) = notes {
        if !result.is_empty() {
            result.push_str(&format!("{indent}///\n"));
        }
        result.push_str(&format!("{indent}/// Notes:\n{indent}///\n"));
        for note in notes {
            result.push_str(&format!("{indent}/// {}\n", note.to_doc_string(&indent)));
        }
    }
    if let Some(examples) = examples {
        result.push_str(&format!(
            "{indent}///\n{indent}/// Examples:\n{indent}///\n"
        ));
        for example in examples {
            result.push_str(&format!("{indent}/// {}\n", example.to_doc_string(&indent)));
        }
    }
    result
}

enum Import {
    HashMap,
    Classes,
    Concepts,
    Defines,
    Events,
    Prototypes,
    Types,
}

impl ToString for Import {
    fn to_string(&self) -> String {
        match self {
            Import::HashMap => "use std::collections::HashMap;".to_owned(),
            Import::Classes => "use super::classes::*;".to_owned(),
            Import::Concepts => "use super::concepts::*;".to_owned(),
            Import::Defines => "use super::defines::*;".to_owned(),
            Import::Events => "use super::events::*;".to_owned(),
            Import::Prototypes => "use super::prototypes::*;".to_owned(),
            Import::Types => "use super::types::*;".to_owned(),
        }
    }
}

fn generate<G: Generate>(list: &[G], imports: Vec<Import>) -> String {
    let mut result = imports
        .iter()
        .map(Import::to_string)
        .collect::<Vec<_>>()
        .join("\n");
    result.push_str("\n\n");
    result.push_str(
        &list
            .iter()
            .map(|p| p.generate(String::new(), false, 0, &mut vec![]))
            .filter(|s| !s.is_empty())
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
            .map(|p| p.generate(name.to_owned(), false, 1, &mut unions))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    for union in unions {
        result.insert_str(0, &format!("{union}\n\n"));
    }
    result.push_str("\n}");
    result
}

fn generate_union(
    name: &str,
    options: &Vec<Type>,
    unions: &mut Vec<String>,
    properties: Option<&Vec<Property>>,
) -> String {
    let mut union = format!("pub enum {name} {{\n");
    let mut fields = HashSet::new();
    for option in options {
        let (has_value, has_struct) = if let Type::Complex(complex) = option {
            match complex.as_ref() {
                ComplexType::Literal {
                    value: _,
                    description,
                } => {
                    union.push_str(&generate_docs(description.as_ref(), None, None, None, 1));
                    (false, false)
                }
                ComplexType::Type { value, description } => {
                    union.push_str(&generate_docs(Some(description), None, None, None, 1));
                    if let Type::Simple(_) = value {
                        (true, false)
                    } else {
                        (false, false)
                    }
                }
                ComplexType::Struct => (false, true),
                _ => (true, false),
            }
        } else {
            (true, false)
        };
        let prefix = if has_struct || name == "LightDefinition" {
            name.to_owned()
        } else {
            format!("{}Union", name)
        };
        let mut result = option.generate(prefix, true, 1, unions);
        let field = result.to_rust_field_name().to_pascal_case();
        let mut added = false;
        if !fields.contains(&field) || name == "Direction" {
            union.push_str(&format!("    {field}"));
            fields.insert(field);
            added = true;
        }
        if has_struct {
            union.push_str(" {\n");
            union.push_str(
                &properties
                    .expect("should have properties")
                    .iter()
                    .map(|p| p.generate(name.to_owned(), true, 2, unions))
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
            union.push_str("\n    }");
        } else if has_value {
            if name == "NoiseNumber" || name == "NoiseExpression" || name == "TriggerEffectUnion" {
                result = format!("Box<{result}>");
            }
            union.push_str(&format!("({result})"));
        }
        if added {
            union.push_str(",\n");
        }
    }
    union.push('}');
    if name == "Direction" {
        union = union.replace("F64", "").replace("/// ", "");
    }
    if !unions
        .iter()
        .any(|u| u.contains(name) && name.ends_with("Union"))
    {
        unions.push(union);
    }
    name.to_owned()
}

pub fn generate_factorio_types(
    factorio_types_path: &str,
    prototype_api: &PrototypeApiFormat,
    runtime_api: &RuntimeApiFormat,
) -> io::Result<()> {
    let mut content = String::from("pub enum FactorioType {\n");
    for s in ["Class", "Concept", "Define", "Event", "Prototype", "Type"] {
        content.push_str(&format!("    {s}({s}),\n"));
    }
    content.push_str("}\n\n");
    content.push_str(&runtime_api.generate_factorio_types());
    content.push_str(&prototype_api.generate_factorio_types());
    fs::write(factorio_types_path, content)
}

pub fn generate_mod(mod_path: &str) -> io::Result<()> {
    let mut content = String::new();
    for s in [
        "classes",
        "concepts",
        "defines",
        "events",
        "prototypes",
        "types",
        "factorio_types",
    ] {
        content.push_str(&format!("mod {s};\npub use {s}::*;\n\n"));
    }
    fs::write(mod_path, content)
}
