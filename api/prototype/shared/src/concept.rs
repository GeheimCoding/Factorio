use crate::basic_member::BasicMember;
use crate::case::Case;
use crate::file_utils::save_file_if_changed;
use crate::property::Property;
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

    pub fn is_builtin(&self) -> bool {
        // self.type_ == Type::Simple(String::from("builtin"))
        self.base
            .name
            .chars()
            .next()
            .map_or(false, char::is_lowercase)
    }

    fn generate_internal(&self) -> anyhow::Result<String> {
        let concept = format!("pub struct {} {{", self.rust_name());
        Ok(format!("{concept}}}"))
    }
}
