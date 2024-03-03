mod generated;

pub use generated::*;
use std::collections::HashMap;

pub fn parse_factorio_type(json: &str) -> Result<FactorioType, serde_json::Error> {
    serde_json::from_str(json)
}

pub fn add_to_lua_objects<'a, T: LuaObject>(
    lua_object: &'a T,
    lua_objects: &mut HashMap<String, &'a dyn LuaObject>,
) {
    let mut lua_object_queue: Vec<&dyn LuaObject> = vec![lua_object];

    while let Some(lua_object) = lua_object_queue.pop() {
        let class_id = lua_object.class_id();
        if lua_objects.contains_key(&class_id) {
            continue;
        }
        lua_objects.insert(class_id, lua_object);

        // TODO: also check maps and lists
        // https://users.rust-lang.org/t/how-to-do-downcasting-from-trait-to-generic-struct/26969
        for (_, field) in lua_object.iter() {
            if let Some(lua_object) = resolve_cycle(field) {
                lua_object_queue.push(lua_object);
            }
        }
    }
}
