mod generated;

use extensions::{LuaObject, Traversable};
pub use generated::*;
use std::collections::HashMap;

pub fn parse_factorio_type(json: &str) -> Result<FactorioType, serde_json::Error> {
    serde_json::from_str(json)
}

pub fn add_to_lua_objects<'a, T: Traversable>(
    traversable: &'a T,
    lua_objects: &mut HashMap<&'a str, &'a dyn LuaObject>,
) {
    let mut queue: Vec<&dyn Traversable> = vec![traversable];
    while let Some(traversable) = queue.pop() {
        queue.extend(traversable.traverse());
        let Some(lua_object) = traversable.to_lua_object() else {
            continue;
        };
        let class_id = lua_object.class_id();
        if lua_objects.contains_key(&class_id) {
            continue;
        }
        lua_objects.insert(class_id, lua_object);
    }
}
