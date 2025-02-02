use crate::basic_member::BasicMember;
use crate::format::{Context, DataType};
use crate::transformation::Transformation;
use crate::type_::{ComplexType, LiteralValue, Type};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Property {
    #[serde(flatten)]
    pub base: BasicMember,
    pub visibility: Option<Vec<String>>,
    pub alt_name: Option<String>,
    #[serde(rename = "override")]
    pub override_: bool,
    #[serde(rename = "type")]
    pub type_: Type,
    pub optional: bool,
    pub default: Option<PropertyDefault>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PropertyDefault {
    String(String),
    Literal(ComplexType),
}

impl Property {
    pub fn generate(&self, prefix: &str, context: &Context) -> Option<(String, Vec<String>)> {
        let is_union_property = matches!(context.context.get(prefix), Some((_, DataType::Union)));
        // README: Adjustment [TODO]
        if prefix == "PrototypeBase" && self.base.name == "type" {
            return None;
        }
        // README: Adjustment [TODO]
        let prefix = format!("{prefix}{}", self.base.name.to_pascal_case());
        let (mut inner, mut additional) = self.type_.generate(&prefix, context);
        if let Some(literal) = self.type_.get_literal_value() {
            if matches!(literal, LiteralValue::String(_)) && self.base.name == "type" {
                return None;
            }
            inner = String::from(literal.to_rust_type());
        }
        let (mut name, other) = self.base.name.to_rust_type(context);
        // README: Adjustment [TODO]
        if prefix.starts_with("TechnologySlotStyleSpecification") && name.contains("offset") {
            inner = String::from("f32");
        }
        // README: Adjustment [TODO]
        assert!(other.is_empty());
        name = name.to_snake_case();
        let rename = if self.base.name != name {
            &format!("#[serde(rename = \"{}\")]", self.base.name)
        } else {
            ""
        };
        let alias = if let Some(alt_name) = &self.alt_name {
            &format!("#[serde(alias = \"{alt_name}\")]")
        } else {
            ""
        };
        let mut comment = String::new();
        let mut serde_default = String::new();
        if let Some(default) = &self.default {
            match default {
                PropertyDefault::String(s) => {
                    comment = format!("\n// default: {s}\n");
                }
                PropertyDefault::Literal(literal) => {
                    // README: Adjustment [TODO]
                    if name == "linked_game_control" && inner.ends_with("LinkedGameControl") {
                        // default value is empty
                    } else if name == "illumination_graphic_index" && inner == "u32" {
                        // default value is -1
                    } else if name == "emissions_per_minute" && inner.contains("HashMap") {
                        // default value is 0
                    }
                    // README: Adjustment [TODO]
                    else {
                        let (default_fn, default_comment) =
                            Self::get_default_fn(&name, &inner, literal, context);
                        if let Some(default_fn) = default_fn {
                            serde_default = format!("#[serde(default = \"default_{name}\")]");
                            additional.push(default_fn)
                        }
                        if let Some(default_comment) = default_comment {
                            comment = format!("\n// default: {default_comment}\n");
                        }
                    }
                }
            }
        }
        if is_union_property && self.type_.should_be_boxed(context) {
            inner = format!("Box<{inner}>");
        }
        if (self.optional && serde_default.is_empty())
        // README: Adjustment [TODO] (e.g. CheckBoxStyleSpecification::disabled_checkmark
            || (name == "filename" && prefix.starts_with("SpriteSource"))
        // README: Adjustment [TODO]
        {
            inner = format!("Option<{inner}>");
        }
        Some((
            format!("{comment}{rename}{alias}{serde_default}{name}: {inner}",),
            additional,
        ))
    }

    fn get_default_fn(
        name: &str,
        return_type: &str,
        complex_type: &ComplexType,
        context: &Context,
    ) -> (Option<String>, Option<String>) {
        let return_value = match complex_type {
            ComplexType::Literal { value, .. } => {
                let r = String::from(
                    return_type
                        .rsplit("::")
                        .next()
                        .expect("must return at least the string back"),
                );
                match value {
                    LiteralValue::String(s) => {
                        if let Some((_, data_type)) = context.context.get(&r) {
                            match data_type {
                                DataType::Union => {
                                    if s.contains("`") {
                                        return (None, Some(String::from(s)));
                                    } else {
                                        format!("{return_type}::{}", s.to_pascal_case())
                                    }
                                }
                                DataType::Struct => {
                                    unreachable!("unexpected struct for default value")
                                }
                                DataType::NewType(_) => format!("String::from(\"{s}\")"),
                            }
                        } else if r == "String" {
                            format!("String::from(\"{s}\")")
                        } else {
                            // README: Adjustment [TODO]
                            if return_type == "PipeConnectionDefinitionConnectionCategory" {
                                format!("{return_type}::String(String::from(\"{s}\"))")
                            } else if return_type == "MiningDrillGraphicsSetCircuitConnectorLayer"
                                || return_type == "CraftingMachineGraphicsSetCircuitConnectorLayer"
                            {
                                format!(
                                    "{return_type}::RenderLayer(crate::types::RenderLayer::{})",
                                    s.to_pascal_case()
                                )
                            }
                            // README: Adjustment [TODO]
                            else {
                                format!("{return_type}::{}", s.to_pascal_case())
                            }
                        }
                    }
                    LiteralValue::Number(n) => {
                        let r = context.context.get(&r);
                        if return_type == "f32"
                            || return_type == "f64"
                            || matches!(r, Some((_, DataType::NewType(s))) if s == "float" || s == "double")
                        {
                            format!("{:.1}", n)
                        } else if matches!(r, Some((_, DataType::NewType(s))) if s == "string") {
                            format!("String::from(\"{n}\")")
                        } else {
                            // README: Adjustment [TODO]
                            if return_type
                                .ends_with("GraphicsSetCircuitConnectorSecondaryDrawOrder")
                            {
                                format!("{return_type}::I8({n})")
                            }
                            // README: Adjustment [TODO]
                            else {
                                n.to_string()
                            }
                        }
                    }
                    LiteralValue::Bool(b) => b.to_string(),
                }
            }
            _ => unreachable!("unexpected default value type: {:?}", complex_type),
        };
        (
            Some(format!(
                "fn default_{name}() -> {return_type} {{ {return_value} }}"
            )),
            None,
        )
    }
}
