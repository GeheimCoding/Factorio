use crate::basic_member::BasicMember;
use crate::file_utils::save_file_if_changed;
use crate::property::Property;
use crate::transformation::Transformation;
use crate::type_::Type;
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
    pub fn generate(&self, path: &Path) -> anyhow::Result<()> {
        let path = &path.join(self.name()).with_extension("rs");
        let concept = self.generate_internal();
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
        // README: Adjustment [TODO]
        && self.rust_name() != "Direction"
        && self.rust_name() != "ComparatorString"
        // README: Adjustment [TODO]
    }

    fn generate_internal(&self) -> String {
        if self.type_.is_struct() {
            self.generate_struct()
        } else if self.type_.is_union() {
            self.generate_enum()
        } else {
            self.generate_new_type()
        }
    }

    fn generate_struct(&self) -> String {
        self.assert_properties();
        let name = self.rust_name();
        let (inner, additional) = self.type_.generate(name, &self.properties);
        format!("pub struct {name}{inner}{}", additional.join(""))
    }

    fn generate_enum(&self) -> String {
        if !self.type_.contains_struct() {
            self.assert_no_properties();
            self.assert_no_parent();
        } else {
            self.assert_properties();
        }
        let (_, mut additional) = self.type_.generate(&self.rust_name(), &self.properties);
        let mut seen: HashSet<String> = HashSet::new();
        additional.retain(|a| seen.insert(a.clone()));
        additional.join("")
    }

    fn generate_new_type(&self) -> String {
        let name = self.rust_name();
        self.assert_no_properties();
        self.assert_no_parent();
        // README: Adjustment [3]
        if name == "DataExtendMethod" {
            assert!(
                matches!(&self.type_, Type::Simple(simple) if simple == &String::from("builtin")),
                "expected builtin type"
            );
            return String::from("pub struct DataExtendMethod;");
        }
        // README: Adjustment [3]
        let (generated, additional) = self.type_.generate(&format!("{name}Variants"), &None);
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
}
