use crate::basic_member::BasicMember;
use crate::define_value::DefineValue;
use crate::file_utils::save_file_if_changed;
use crate::pascal_case::PascalCase;
use serde::Deserialize;
use std::io;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Define {
    #[serde(flatten)]
    pub base: BasicMember,
    pub values: Option<Vec<DefineValue>>,
    pub subkeys: Option<Vec<Define>>,
}

impl Define {
    pub fn generate(&self, path: &Path) -> io::Result<()> {
        let path = &path.join(&self.base.name).with_extension("rs");
        let define = self.generate_internal()?;
        save_file_if_changed(path, &define)
    }

    pub fn name(&self) -> &str {
        &self.base.name
    }

    pub fn rust_name(&self) -> String {
        self.base.name.to_pascal_case()
    }

    fn generate_internal(&self) -> io::Result<String> {
        let mut define = format!("pub enum {}{{", self.rust_name());
        if let Some(values) = &self.values {
            define.push_str(&values.iter().map(DefineValue::generate).collect::<String>());
        }
        if let Some(subkeys) = &self.subkeys {
            for sub in subkeys {
                let sub_name = sub.rust_name();
                define.push_str(&format!("{}({}),", sub_name, sub_name));
                // TODO: Adjustment [1]
                if self.rust_name() == "CargoLandingPad" && sub_name == "ExclusiveMode" {
                    continue;
                }
                // TODO: Adjustment [1]
                define.insert_str(0, &sub.generate_internal()?);
            }
        }
        Ok(format!("{define}}}"))
    }
}
