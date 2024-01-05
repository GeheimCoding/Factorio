use self::{
    prototype::property::Property,
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

fn generate<G: Generate>(list: &[G]) -> String {
    let mut result = String::new();
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
    for option in options {
        let result = option.generate(name.to_owned(), true, 1, unions);
        let (has_value, has_struct) = if let Type::Complex(complex) = option {
            match complex.as_ref() {
                ComplexType::Literal {
                    value: _,
                    description,
                } => {
                    union.push_str(&generate_docs(description.as_ref(), None, None, 1));
                    (false, false)
                }
                ComplexType::Type {
                    value: _,
                    description,
                } => {
                    union.push_str(&generate_docs(Some(description), None, None, 1));
                    (false, false)
                }
                ComplexType::Struct => (false, true),
                _ => (true, false),
            }
        } else {
            (true, false)
        };
        union.push_str(&format!(
            "    {}",
            result.to_rust_field_name().to_pascal_case()
        ));
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
            union.push_str(&format!("({result})"));
        }
        union.push_str(",\n");
    }
    union.push('}');
    unions.push(union);
    name.to_owned()
}