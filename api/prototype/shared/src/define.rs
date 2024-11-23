use crate::basic_member::BasicMember;
use crate::define_value::DefineValue;
use crate::derive::Derive;
use crate::file_utils::save_file_if_changed;
use crate::lua_value::{LuaValue, State};
use crate::pascal_case::PascalCase;
use serde::Deserialize;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Define {
    #[serde(flatten)]
    pub base: BasicMember,
    pub values: Option<Vec<DefineValue>>,
    pub subkeys: Option<Vec<Define>>,
}

#[derive(PartialEq)]
enum DeriveType {
    None,
    SerdeRepr,
    SerdeRename,
}

impl Define {
    pub fn generate(
        &self,
        path: &Path,
        lua_defines: &HashMap<String, LuaValue>,
    ) -> anyhow::Result<()> {
        let path = &path.join(&self.base.name).with_extension("rs");
        let define = self.generate_internal("", lua_defines)?;
        save_file_if_changed("defines", path, &define)
    }

    pub fn name(&self) -> &str {
        &self.base.name
    }

    pub fn rust_name(&self) -> String {
        self.base.name.to_pascal_case()
    }

    fn generate_internal(
        &self,
        lua_define_key: &str,
        lua_defines: &HashMap<String, LuaValue>,
    ) -> anyhow::Result<String> {
        let mut define = format!("pub enum {}{{", self.rust_name());
        let mut other = String::new();
        let mut derive_type = DeriveType::None;
        let lua_define_key = if lua_define_key.is_empty() {
            &self.base.name
        } else {
            &format!("{lua_define_key}.{}", self.base.name)
        };
        let parent_lua_value = lua_defines.get(lua_define_key);
        // README: Adjustment [2]
        if let Some(LuaValue::State(State::ContainsDuplicates)) = lua_defines.get(lua_define_key) {
            assert!(self.subkeys.is_none(), "unexpected subkeys");
            let (serde, variants) =
                &self.generate_values_with_duplicates(lua_define_key, lua_defines);
            other.push_str(serde);
            define.push_str(variants);
        }
        // README: Adjustment [2]
        else if let Some(values) = &self.values {
            assert!(self.subkeys.is_none(), "unexpected subkeys");
            define.push_str(
                &values
                    .iter()
                    .map(|value| {
                        let lua_define =
                            lua_defines.get(&format!("{lua_define_key}.{}", value.name));
                        if let Some(LuaValue::String(_)) = lua_define {
                            derive_type = DeriveType::SerdeRename;
                        } else if let Some(LuaValue::Number(_)) = lua_define {
                            if parent_lua_value != Some(&LuaValue::State(State::Lookup)) {
                                derive_type = DeriveType::SerdeRepr;
                            }
                        }
                        value.generate(lua_define, parent_lua_value)
                    })
                    .collect::<String>(),
            );
        } else if let Some(subkeys) = &self.subkeys {
            for sub in subkeys {
                let sub_name = sub.rust_name();
                define.push_str(&format!("{}({}),", sub_name, sub_name));
                // README: Adjustment [1]
                if self.rust_name() == "CargoLandingPad" && sub_name == "ExclusiveMode" {
                    continue;
                }
                // README: Adjustment [1]
                other.push_str(&sub.generate_internal(lua_define_key, lua_defines)?);
            }
        }
        let derive = match derive_type {
            DeriveType::None => Derive::new(),
            DeriveType::SerdeRepr => Derive::new().with_serde_repr(),
            DeriveType::SerdeRename => Derive::new().with_serde().with_kebab_case(),
        };
        Ok(format!("{derive}{define}}} {other}"))
    }

    // README: Adjustment [2]
    fn generate_values_with_duplicates(
        &self,
        parent_lua_define_key: &str,
        lua_defines: &HashMap<String, LuaValue>,
    ) -> (String, String) {
        let mut values = HashMap::new();
        self.values
            .as_ref()
            .expect("expected values")
            .iter()
            .for_each(|value| {
                let key = format!("{parent_lua_define_key}.{}", value.name);
                let lua_value = lua_defines
                    .get(&key)
                    .expect(&format!("expected define for key {key}"));
                if let LuaValue::Number(number) = lua_value {
                    values
                        .entry(number.to_string())
                        .or_insert(vec![])
                        .push(value.name.to_pascal_case());
                } else {
                    unimplemented!("expected a number");
                }
            });
        let mut values = values.iter().collect::<Vec<_>>();
        values.sort_by(|a, b| a.0.cmp(&b.0));

        let rust_name = self.rust_name();
        let mut serde = String::new();
        let mut variants = String::new();
        let mut return_variants = String::new();
        values.iter().for_each(|(key, values)| {
            if values.len() == 1 {
                let value = values.get(0).expect("must exist");
                return_variants.push_str(&format!("{value},"));
                variants.push_str(&format!("{key} => Ok({rust_name}::{value}),"));
            } else {
                return_variants.push_str(&format!(
                    "Value{key}(std::collections::HashSet<Value{key}>),"
                ));
                variants.push_str(&format!(
                    "{key} => Ok({rust_name}::Value{key}(std::collections::HashSet::from([{}]))),",
                    values
                        .iter()
                        .map(|value| format!("Value{key}::{value},"))
                        .collect::<String>()
                ));
                serde.push_str(&format!(
                    "{}pub enum Value{key}{{ {}, }}",
                    Derive::new().with_hash(),
                    values.join(",")
                ));
            }
        });
        serde.push_str(&format!(
            r#"
            impl<'de> serde::Deserialize<'de> for {rust_name} {{
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {{
                    match <u16 as serde::Deserialize>::deserialize(deserializer)? {{
                        {variants}
                        other => Err(serde::de::Error::custom(format!("unexpected value: {{other}}"))),
                    }}
                }}
            }}
        "#
        ));
        (serde, return_variants)
    }
    // README: Adjustment [2]
}
