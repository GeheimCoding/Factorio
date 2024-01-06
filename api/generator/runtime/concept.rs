#![allow(unused)]
use serde::Deserialize;

use crate::generator::{
    generate_docs,
    type_::{ComplexType, Tuple, Type},
    Generate, StringTransformation,
};

#[derive(Debug, Deserialize)]
pub struct Concept {
    /// The name of the concept.
    name: String,
    /// The order of the concept as shown in the HTML.
    order: u16,
    /// The text description of the concept.
    description: String,
    /// A list of strings containing additional information about the concept.
    notes: Option<Vec<String>>,
    /// A list of strings containing example code and explanations.
    examples: Option<Vec<String>>,
    /// The type of the concept.
    #[serde(rename = "type")]
    type_: Type,
}

impl Generate for Concept {
    fn generate(
        &self,
        prefix: String,
        enum_variant: bool,
        indent: usize,
        unions: &mut Vec<String>,
    ) -> String {
        let mut result = generate_docs(
            Some(&self.description),
            None,
            self.notes.as_ref(),
            self.examples.as_ref(),
            indent,
        );
        let name = self.name.to_pascal_case();
        let mut unions = vec![];
        if is_new_tpye(&self.type_) {
            result.push_str(&format!(
                "pub type {} = {};",
                name.clone(),
                self.type_
                    .generate(format!("{name}Union"), enum_variant, indent, &mut unions)
            ));
        }
        for union in unions {
            result.insert_str(0, &format!("{union}\n\n"));
        }
        result
    }
}

fn is_new_tpye(type_: &Type) -> bool {
    match type_ {
        Type::Simple(_) => true,
        Type::Complex(c) => match c.as_ref() {
            ComplexType::Array { value: _ }
            | ComplexType::Dictionary { key: _, value: _ }
            | ComplexType::Tuple(Tuple::Tuple { values: _ })
            | ComplexType::Literal {
                value: _,
                description: _,
            } => true,
            ComplexType::Type {
                value,
                description: _,
            }
            | ComplexType::LuaLazyLoadedValue { value } => is_new_tpye(value),
            _ => false,
        },
    }
}
