#![allow(unused)]
use std::collections::HashSet;

use serde::Deserialize;

use crate::generator::{
    generate_docs,
    type_::{ComplexType, Tuple, Type},
    Generate, Macro, StringTransformation,
};

pub const SETTINGS: [(&str, &str); 11] = [
    ("LuaMapSettings", "MapSettings"),
    ("LuaDifficultySettings", "DifficultySettings"),
    ("LuaGameViewSettings", "GameViewSettings"),
    (
        "LuaMapSettings.enemy_evolution",
        "EnemyEvolutionMapSettings",
    ),
    (
        "LuaMapSettings.enemy_expansion",
        "EnemyExpansionMapSettings",
    ),
    ("LuaMapSettings.path_finder", "PathFinderMapSettings"),
    ("LuaMapSettings.pollution", "PollutionMapSettings"),
    ("LuaMapSettings.steering", "SteeringMapSettings"),
    ("LuaMapSettings.steering.default", "SteeringMapSetting"),
    ("LuaMapSettings.steering.moving", "SteeringMapSetting"),
    ("LuaMapSettings.unit_group", "UnitGroupMapSettings"),
];

#[derive(Debug, Deserialize)]
pub struct Concept {
    /// The name of the concept.
    pub name: String,
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
    pub type_: Type,
}

impl Generate for Concept {
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
        let name = self.name.to_pascal_case();
        let mut unions = vec![];
        let mut generate_unions = true;
        if is_new_tpye(&self.type_) {
            result.push_str(&format!(
                "pub type {} = {};",
                name.clone(),
                self.type_.generate(
                    format!("{name}Union"),
                    enum_variant,
                    indent,
                    &mut unions,
                    class_names
                )
            ));
        } else if name == "ComparatorString" {
            result.push_str("pub type ComparatorString = String;");
        } else if is_union_with_table_or_tuple(&self.type_) {
            if let Type::Complex(c) = &self.type_ {
                result.push_str(&match c.as_ref() {
                    ComplexType::Union {
                        options,
                        full_format,
                    } => options[0].generate(
                        name.clone(),
                        enum_variant,
                        indent,
                        &mut unions,
                        class_names,
                    ),
                    _ => panic!("should be union"),
                });
            } else {
                panic!("should be complex");
            }
        } else {
            let mut type_ =
                self.type_
                    .generate(name.clone(), enum_variant, indent, &mut unions, class_names);
            if type_ != name {
                result.push_str(&type_);
            } else {
                generate_unions = false;
                if unions.len() > 1 {
                    panic!("expected only one union");
                }
                result.push_str(&unions[0]);
            }
        }
        if generate_unions {
            for union in unions {
                result.insert_str(0, &format!("{union}\n\n"));
            }
        }
        if self.name == "BlueprintEntity" {
            result.insert_str(
                0,
                &format!(
                    "{}\npub struct BlueprintCircuitConnection;\n\n",
                    Macro::DebugDeserialize.to_string()
                ),
            );
            result.insert_str(
                0,
                &format!(
                    "{}\npub struct BlueprintControlBehavior;\n\n",
                    Macro::DebugDeserialize.to_string()
                ),
            );
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

fn is_union_with_table_or_tuple(type_: &Type) -> bool {
    if let Type::Complex(c) = type_ {
        let c = c.as_ref();
        if let ComplexType::Union {
            options,
            full_format,
        } = c
        {
            return options.len() == 2
                && is_table_or_tuple(&options[0])
                && is_table_or_tuple(&options[1]);
        }
    }
    false
}

fn is_table_or_tuple(type_: &Type) -> bool {
    if let Type::Complex(c) = type_ {
        return match c.as_ref() {
            ComplexType::Table {
                parameters,
                variant_parameter_groups,
                variant_parameter_description,
            }
            | ComplexType::Tuple(Tuple::Table {
                parameters,
                variant_parameter_groups,
                variant_parameter_description,
            }) => true,
            _ => false,
        };
    }
    false
}
