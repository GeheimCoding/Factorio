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
            let (serde, variants) =
                &self.generate_values_with_duplicates(lua_define_key, lua_defines);
            define.insert_str(0, serde);
            define.push_str(variants);
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
                    "{key} => {{use std::collections::HashSet;let mut variants = vec![];{}Ok({rust_name}::Value{key}(HashSet::from_iter(variants)))}}",
                    values
                        .iter()
                        .map(|value| format!("variants.push(Value{key}::{value});"))
                        .collect::<String>()
                ));
                serde.push_str(&format!(
                    "#[derive(Eq, Hash, PartialEq)]pub enum Value{key}{{ {}, }}",
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
        // TODO: implement serde for either repr or rename (String - check casing?) or lookup
        (serde, return_variants)
    }
    // README: Adjustment [2]
}
