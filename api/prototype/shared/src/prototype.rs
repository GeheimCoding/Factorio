use crate::basic_member::BasicMember;
use crate::custom_properties::CustomProperties;
use crate::file_utils::save_file_if_changed;
use crate::format::{Context, DataType};
use crate::property::Property;
use crate::transformation::Transformation;
use crate::type_::{ComplexType, Type};
use serde::Deserialize;
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Prototype {
    #[serde(flatten)]
    pub base: BasicMember,
    pub visibility: Option<Vec<String>>,
    pub parent: Option<String>,
    #[serde(rename = "abstract")]
    pub abstract_: bool,
    pub typename: Option<String>,
    pub instance_limit: Option<u64>,
    pub deprecated: bool,
    pub properties: Vec<Property>,
    pub custom_properties: Option<CustomProperties>,
}

impl Prototype {
    pub fn generate(&self, path: &Path, context: &Context) -> anyhow::Result<()> {
        let path = &path.join(self.name()).with_extension("rs");
        let prototype = self.generate_internal(context);
        save_file_if_changed("prototypes", path, &prototype)
    }

    pub fn name(&self) -> String {
        self.base.name.to_snake_case()
    }

    pub fn rust_name(&self) -> &str {
        &self.base.name
    }

    fn generate_internal(&self, context: &Context) -> String {
        if let Some((_, DataType::Struct)) = context.context.get(self.rust_name()) {
            self.generate_struct(context)
        } else {
            unreachable!(
                "expected to find struct in context for {}",
                self.rust_name()
            )
        }
    }

    fn generate_struct(&self, context: &Context) -> String {
        let name = self.rust_name();
        let (inner, mut additional) =
            Type::Complex(Box::new(ComplexType::Struct)).generate(name, context);
        let mut seen: HashSet<String> = HashSet::new();
        additional.retain(|a| seen.insert(a.clone()));
        format!("pub struct {name}{inner}{}", additional.join(""))
    }
}
