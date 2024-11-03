use crate::basic_member::BasicMember;
use crate::define_value::DefineValue;
use crate::pascal_case::PascalCase;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Define {
    #[serde(flatten)]
    base: BasicMember,
    values: Option<Vec<DefineValue>>,
    subkeys: Option<Vec<Define>>,
}

impl Define {
    pub fn generate(&self) -> String {
        self.generate_with_prefix("")
    }

    fn generate_with_prefix(&self, prefix: &str) -> String {
        let name = format!("{prefix}{}", self.generate_name());
        let mut define = String::from(format!("pub enum {name}{{"));

        if let Some(values) = &self.values {
            define.push_str(&values.iter().map(DefineValue::generate).collect::<String>());
        }
        if let Some(subkeys) = &self.subkeys {
            for sub in subkeys {
                let sub_name = sub.generate_name();
                define.push_str(format!("{}({}{}),", sub_name, name, sub_name).as_str());
                define.insert_str(0, &sub.generate_with_prefix(&name));
            }
        }
        format!("{define}}}")
    }

    fn generate_name(&self) -> String {
        self.base.name.to_pascal_case()
    }
}
