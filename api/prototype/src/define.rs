use crate::basic_member::BasicMember;
use crate::define_value::DefineValue;
use crate::pascal_case::PascalCase;
use serde::Deserialize;
use std::path::Path;
use std::process::Command;
use std::{fs, io};

#[derive(Debug, Deserialize)]
pub struct Define {
    #[serde(flatten)]
    base: BasicMember,
    values: Option<Vec<DefineValue>>,
    subkeys: Option<Vec<Define>>,
}

impl Define {
    pub fn generate(&self, path: &Path) -> io::Result<()> {
        let path = &path.join(&self.base.name).with_extension("rs");
        let define = self.generate_internal()?;

        fs::write(path, define)?;
        Command::new("rustfmt").arg(path).spawn()?;
        Ok(())
    }

    fn generate_internal(&self) -> io::Result<String> {
        let mut define = format!("pub enum {}{{", self.generate_name());
        if let Some(values) = &self.values {
            define.push_str(&values.iter().map(DefineValue::generate).collect::<String>());
        }
        if let Some(subkeys) = &self.subkeys {
            for sub in subkeys {
                let sub_name = sub.generate_name();
                define.push_str(&format!("{}({}),", sub_name, sub_name));
                define.insert_str(0, &sub.generate_internal()?);
            }
        }
        Ok(format!("{define}}}"))
    }

    fn generate_name(&self) -> String {
        self.base.name.to_pascal_case()
    }
}
