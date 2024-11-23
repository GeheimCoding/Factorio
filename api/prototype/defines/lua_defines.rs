use anyhow::anyhow;
use mlua::prelude::LuaError;
use mlua::{Lua, Value};
use shared::lua_value::{LuaValue, State};
use std::collections::{HashMap, HashSet};
use std::fs;

pub fn parse_lua_defines(path: &str) -> anyhow::Result<HashMap<String, LuaValue>> {
    let lua = Lua::new();
    let defines = fs::read_to_string(path)?;
    let root = lua
        .load(defines)
        .eval::<Value>()
        .map_err(|e| anyhow!("{e}"))?;

    let mut defines = HashMap::new();
    traverse(String::new(), root, &mut defines)?;

    Ok(defines)
}

fn traverse(
    prefix: String,
    value: Value,
    defines: &mut HashMap<String, LuaValue>,
) -> anyhow::Result<LuaValue> {
    let lua_value;
    if let Some(table) = value.as_table() {
        let next_prefix = if prefix.is_empty() {
            String::new()
        } else {
            format!("{prefix}.")
        };
        let mut values = HashSet::new();
        let mut state = State::AllDifferent;
        table
            .for_each(|k: String, v| {
                let value = traverse(format!("{next_prefix}{k}"), v, defines)
                    .map_err(|e| LuaError::external(e))?;
                if let LuaValue::State(_) = value {
                } else if !values.insert(value) {
                    // README: Adjustment [2]
                    state = State::ContainsDuplicates;
                    // README: Adjustment [2]
                }
                Ok(())
            })
            .map_err(|e| anyhow!("{e}"))?;
        if values.len() == 1 && values.contains(&LuaValue::Number(0)) {
            state = State::Lookup;
        }
        lua_value = LuaValue::State(state);
    } else if let Some(number) = value.as_integer() {
        lua_value = LuaValue::Number(number);
    } else if let Some(string) = value.as_string() {
        lua_value = LuaValue::String(string.to_string_lossy());
    } else {
        unimplemented!("unexpected value: {:?}", value)
    }
    defines.insert(prefix, lua_value.clone());
    Ok(lua_value)
}
