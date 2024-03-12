#![allow(unused)]
use std::collections::HashSet;

use serde::Deserialize;

use crate::generator::{generate_docs, Generate, Macro};

use super::{attribute::Attribute, method::Method, operator::Operator};

#[derive(Debug, Deserialize)]
pub struct Class {
    /// The name of the class.
    pub name: String,
    /// The order of the class as shown in the HTML.
    order: u16,
    /// The text description of the class.
    description: String,
    /// A list of strings containing additional information about the class.
    notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    examples: Option<Vec<String>>,
    /// The methods that are part of the class.
    methods: Vec<Method>,
    /// The attributes that are part of the class.
    pub attributes: Vec<Attribute>,
    /// A list of operators on the class. They are called `call`, `index`, or `length` and have the format of either a [Method](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-runtime.html#Method) or an [Attribute](https://lua-api.factorio.com/1.1.101/auxiliary/json-docs-runtime.html#Attribute).
    operators: Vec<Operator>,
    /// Whether the class is never itself instantiated, only inherited from.
    #[serde(rename = "abstract")]
    abstract_: bool,
    /// A list of the names of the classes that his class inherits from.
    base_classes: Option<Vec<String>>,
}

impl Generate for Class {
    fn generate(
        &self,
        prefix: String,
        enum_variant: bool,
        indent: usize,
        unions: &mut Vec<String>,
        class_names: &HashSet<String>,
    ) -> String {
        let mut result = generate_docs(
            Some(&self.description),
            None,
            self.notes.as_ref(),
            self.examples.as_ref(),
            indent,
        );
        let mut unions = vec![];
        result.push_str(&format!(
            "{}\npub struct {} {{\n    #[lua_object]\n    pub class_id: String,\n",
            Macro::DebugDeserialize.to_string(),
            self.name
        ));
        if let Some(bases) = &self.base_classes {
            for base in bases {
                let type_ = if base == "LuaControl" {
                    format!("Box<{base}>")
                } else {
                    base.to_owned()
                };
                // TODO: convert all base attributes with lua instead
                result.push_str(&format!(
                    "    pub parent_{}: Option<{}>,\n",
                    to_snake_case(base),
                    type_
                ));
            }
        }
        result.push_str(
            &self
                .attributes
                .iter()
                .map(|a| {
                    a.generate(
                        self.name.clone(),
                        enum_variant,
                        indent + 1,
                        &mut unions,
                        class_names,
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"),
        );
        result.push_str("\n}");
        for union in unions {
            result.insert_str(0, &format!("{union}\n\n"));
        }
        result
    }
}

fn to_snake_case(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let mut chars = s.chars();
    let mut snake_case = String::from(
        chars
            .next()
            .expect("there should be at least one character")
            .to_ascii_lowercase(),
    );
    for c in chars {
        if c.is_ascii_uppercase() {
            snake_case.push('_');
            snake_case.push(c.to_ascii_lowercase());
        } else {
            snake_case.push(c);
        }
    }
    snake_case
}
