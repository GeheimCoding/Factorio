mod generated;

use extensions::{LuaObject, Traversable};
pub use generated::*;
use std::collections::HashMap;
use std::mem;

pub type LuaObjects<'a> = HashMap<&'a str, &'a dyn LuaObject>;

pub struct Runtime {
    lua_objects: LuaObjects<'static>,
    factorio_types: Vec<Box<FactorioType>>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            lua_objects: Default::default(),
            factorio_types: vec![],
        }
    }

    pub fn lua_objects(&self) -> &LuaObjects {
        &self.lua_objects
    }

    pub fn factorio_types(&self) -> &Vec<Box<FactorioType>> {
        &self.factorio_types
    }
}

pub fn parse_factorio_type(
    json: &str,
    runtime: &mut Runtime,
) -> Result<Option<Box<FactorioType>>, serde_json::Error> {
    let factorio_type = Box::new(serde_json::from_str(json)?);
    let lua_object_count = runtime.lua_objects.len();

    // SAFETY: if the lua objects grow, we add factorio_type to the vec, so the reference
    // is always valid, because it references the boxed value within the struct itself.
    add_to_lua_objects(&factorio_type, unsafe {
        mem::transmute(&mut runtime.lua_objects)
    });
    if runtime.lua_objects.len() > lua_object_count {
        runtime.factorio_types.push(factorio_type);
    } else {
        return Ok(Some(factorio_type));
    }
    Ok(None)
}

fn add_to_lua_objects<'a, T: Traversable>(traversable: &'a T, lua_objects: &mut LuaObjects<'a>) {
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
