use crate::basic_member::BasicMember;
use crate::file_utils::save_file_if_changed;
use crate::property::Property;
use crate::transformation::Transformation;
use crate::type_::Type;
use serde::Deserialize;
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
        let concept = self.generate_internal()?;
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

    fn generate_internal(&self) -> anyhow::Result<String> {
        if let Type::Simple(simple) = &self.type_ {
            return Ok(self.generate_simple_type(simple));
        }
        let concept = format!("pub struct {} {{", self.rust_name());
        Ok(format!("{concept}}}"))
    }

    fn generate_simple_type(&self, simple: &String) -> String {
        let name = self.rust_name();
        assert!(
            self.properties.is_none(),
            "expected no properties for '{name}' with type '{simple}'"
        );
        assert!(
            self.parent.is_none(),
            "expected no parent for '{name}' with type '{simple}'"
        );
        // README: Adjustment [3]
        if name == "DataExtendMethod" {
            assert_eq!(simple, "builtin", "expected builtin type");
            return String::from("pub struct DataExtendMethod;");
        }
        // README: Adjustment [3]
        format!("pub type {name} = {};", simple.to_rust_type())
    }
}
