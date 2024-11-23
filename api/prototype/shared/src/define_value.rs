use crate::lua_value::{LuaValue, State};
use crate::pascal_case::PascalCase;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DefineValue {
    pub name: String,
    pub order: u16,
    pub description: String,
}

impl DefineValue {
    pub fn generate(
        &self,
        lua_value: Option<&LuaValue>,
        parent_lua_value: Option<&LuaValue>,
    ) -> String {
        if let Some(LuaValue::Number(n)) = lua_value {
            if let Some(LuaValue::State(State::AllDifferent)) = parent_lua_value {
                return format!("{} = {n},", self.name.to_pascal_case());
            }
        }
        format!("{},", self.name.to_pascal_case())
    }
}
