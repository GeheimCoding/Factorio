use crate::basic_member::BasicMember;
use crate::context::Context;
use crate::file_utils::save_file_if_changed;
use crate::parameter::Parameter;
use crate::transformation::Transformation;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Event {
    #[serde(flatten)]
    pub base: BasicMember,
    pub data: Vec<Parameter>,
    pub filter: Option<String>,
}

impl Event {
    pub fn generate(&self, path: &Path, context: &Context) -> anyhow::Result<()> {
        let path = &path.join(self.name()).with_extension("rs");
        let event = self.generate_internal(context);
        save_file_if_changed("events", path, &event)
    }

    pub fn name(&self) -> String {
        self.base.name.to_snake_case()
    }

    pub fn rust_name(&self) -> String {
        self.base.name.to_pascal_case()
    }

    fn generate_internal(&self, _context: &Context) -> String {
        let name = self.rust_name();
        let inner = r#"{
            // todo!()
        }"#;
        format!("#[derive(Debug, serde::Deserialize)]pub struct {name}{inner}",)
    }
}
