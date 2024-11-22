use crate::basic_member::BasicMember;
use crate::define_value::DefineValue;
use crate::file_utils::save_file_if_changed;
use crate::lua_value::{LuaValue, State};
use crate::pascal_case::PascalCase;
use serde::Deserialize;
use std::collections::HashMap;
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
    pub fn generate(&self, path: &Path, lua_defines: &HashMap<String, LuaValue>) -> io::Result<()> {
        let path = &path.join(&self.base.name).with_extension("rs");
        let define = self.generate_internal(String::new(), lua_defines)?;
        save_file_if_changed(path, &define)
    }

    pub fn name(&self) -> &str {
        &self.base.name
    }

    pub fn rust_name(&self) -> String {
        self.base.name.to_pascal_case()
    }

    fn generate_internal(
        &self,
        lua_define_key: String,
        lua_defines: &HashMap<String, LuaValue>,
    ) -> io::Result<String> {
        let mut define = format!("pub enum {}{{", self.rust_name());
        let lua_define_key = if lua_define_key.is_empty() {
            &self.base.name
        } else {
            &format!("{lua_define_key}.{}", self.base.name)
        };
        // README: Adjustment [2]
        if let Some(LuaValue::State(State::ContainsDuplicates)) = lua_defines.get(lua_define_key) {
            assert!(self.subkeys.is_none(), "unexpected subkeys");
            define.push_str(&self.generate_values_with_duplicates(lua_define_key, lua_defines))
        }
        // README: Adjustment [2]
        else if let Some(values) = &self.values {
            assert!(self.subkeys.is_none(), "unexpected subkeys");
            define.push_str(&values.iter().map(DefineValue::generate).collect::<String>());
        } else if let Some(subkeys) = &self.subkeys {
            for sub in subkeys {
                let sub_name = sub.rust_name();
                define.push_str(&format!("{}({}),", sub_name, sub_name));
                // README: Adjustment [1]
                if self.rust_name() == "CargoLandingPad" && sub_name == "ExclusiveMode" {
                    continue;
                }
                // README: Adjustment [1]
                define.insert_str(
                    0,
                    &sub.generate_internal(lua_define_key.clone(), lua_defines)?,
                );
            }
        }
        Ok(format!("{define}}}"))
    }

    // README: Adjustment [2]
    fn generate_values_with_duplicates(
        &self,
        parent_lua_define_key: &str,
        _lua_defines: &HashMap<String, LuaValue>,
    ) -> String {
        // TODO: collect values hashmap -> if more than one -> create ValueX enum
        // TODO: implement serde for either repr or rename (String - check casing?) or manual with duplicates or lookup
        // TODO: keep reproducible order -> order by value (also assert value is number)
        println!("cargo::warning={parent_lua_define_key}");
        String::from("TODO")

        // impl<'de> serde::Deserialize<'de> for Define {
        //     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        //     where
        //         D: serde::Deserializer<'de>,
        //     {
        //         match <u8 as serde::Deserialize>::deserialize(deserializer)? {
        //             1 => Ok(Define),
        //             other => Err(serde::de::Error::custom(format!("unexpected value: {other}"))),
        //         }
        //     }
        // }
    }
    // README: Adjustment [2]
}
