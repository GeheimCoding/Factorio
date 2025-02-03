use crate::basic_member::BasicMember;
use crate::file_utils::save_file_if_changed;
use crate::format::{Context, DataType};
use crate::property::Property;
use crate::transformation::Transformation;
use crate::type_::{LiteralValue, Type};
use serde::Deserialize;
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Concept {
    #[serde(flatten)]
    pub base: BasicMember,
    pub parent: Option<String>,
    #[serde(rename = "abstract")]
    pub abstract_: bool,
    pub inline: bool,
    #[serde(rename = "type")]
    pub type_: Type,
    pub properties: Option<Vec<Property>>,
}

impl Concept {
    pub fn generate(&self, path: &Path, context: &Context) -> anyhow::Result<()> {
        let path = &path.join(self.name()).with_extension("rs");
        let concept = self.generate_internal(context);
        save_file_if_changed("types", path, &concept)
    }

    pub fn name(&self) -> String {
        self.base.name.to_snake_case()
    }

    pub fn rust_name(&self) -> &str {
        &self.base.name
    }

    pub fn should_be_generated(&self) -> bool {
        self.base
            .name
            .chars()
            .next()
            .map_or(false, char::is_uppercase)
            && !self.inline
    }

    pub fn generate_internal(&self, context: &Context) -> String {
        if let Some((_, data_type)) = context.context.get(self.rust_name()) {
            match data_type {
                DataType::Struct => self.generate_struct(context),
                DataType::Union => self.generate_enum(context),
                DataType::NewType(_) => self.generate_new_type(context),
            }
        } else {
            unreachable!("expected to find context for {}", self.rust_name())
        }
    }

    pub fn get_tagged_key(&self) -> Option<&String> {
        self.properties.as_ref().and_then(|properties| {
            for property in properties
                .iter()
                .filter(|property| property.base.name == "type")
            {
                if let Some(LiteralValue::String(value)) = property.type_.get_literal_value() {
                    return Some(value);
                }
            }
            None
        })
    }

    fn generate_struct(&self, context: &Context) -> String {
        self.assert_properties();
        let name = self.rust_name();
        let (inner, mut additional) = self.type_.generate(name, context);
        let mut seen: HashSet<String> = HashSet::new();
        additional.retain(|a| seen.insert(a.clone()));

        let derive_hash = if context.hash_keys.contains(name) {
            ", PartialEq, Eq, Hash"
        } else {
            ""
        };
        format!(
            "#[derive(Debug, serde::Deserialize{derive_hash})]pub struct {name}{inner}{}",
            additional.join("")
        )
    }

    fn generate_enum(&self, context: &Context) -> String {
        // README: Adjustment [4]
        if self.rust_name() == "Direction" {
            return Self::generate_direction();
        }
        // README: Adjustment [4]
        // README: Adjustment [5]
        if self.rust_name() == "ComparatorString" {
            return Self::generate_comparator_string();
        }
        // README: Adjustment [5]
        if !self.type_.contains_struct() {
            self.assert_no_properties();
            self.assert_no_parent();
        } else {
            self.assert_properties();
        }
        let (_, mut additional) = self.type_.generate(self.rust_name(), context);
        let mut seen: HashSet<String> = HashSet::new();
        additional.retain(|a| seen.insert(a.clone()));
        additional.join("")
    }

    fn generate_new_type(&self, context: &Context) -> String {
        let name = self.rust_name();
        self.assert_no_properties();
        self.assert_no_parent();
        // README: Adjustment [3]
        if name == "DataExtendMethod" {
            assert!(
                matches!(&self.type_, Type::Simple(simple) if simple == &String::from("builtin")),
                "expected builtin type"
            );
            return String::from(
                "#[derive(Debug, serde::Deserialize)]pub struct DataExtendMethod;",
            );
        }
        // README: Adjustment [3]
        // README: Adjustment [TODO]
        if name == "MapTick" {
            return String::from("pub type MapTick = f64;");
        }
        // README: Adjustment [TODO]
        let (generated, additional) = self.type_.generate(name, context);
        format!("pub type {name} = {generated};{}", additional.join(""))
    }

    fn assert_properties(&self) {
        assert!(
            self.properties.is_some(),
            "expected properties for '{}'",
            self.rust_name()
        );
    }

    fn assert_no_properties(&self) {
        assert!(
            self.properties.is_none(),
            "expected no properties for '{}'",
            self.rust_name()
        );
    }

    fn assert_no_parent(&self) {
        assert!(
            self.parent.is_none(),
            "expected no parent for '{}'",
            self.rust_name()
        );
    }

    // README: Adjustment [4]
    fn generate_direction() -> String {
        String::from("pub type Direction = crate::defines::Direction;")
    }
    // README: Adjustment [4]

    // README: Adjustment [5]
    fn generate_comparator_string() -> String {
        String::from(
            r#"#[derive(Debug, serde::Deserialize)]
            pub enum ComparatorString {
                #[serde(rename = "=")]
                EqualTo,
                #[serde(rename = ">")]
                GreaterThan,
                #[serde(rename = "<")]
                LesserThan,
                #[serde(rename = ">=")]
                #[serde(alias = "≥")]
                GreaterThanOrEqualTo,
                #[serde(rename = "<=")]
                #[serde(alias = "≤")]
                LessThanOrEqualTo,
                #[serde(rename = "!=")]
                #[serde(alias = "≠")]
                NotEqualTo,
            }"#,
        )
    }
    // README: Adjustment [5]
}
