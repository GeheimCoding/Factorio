use api::{parse_factorio_type, FactorioType};
use extensions::{LuaObject, Traversable};
use std::collections::HashMap;
use std::mem;

use anyhow::{Context, Result};

pub type LuaObjects<'a> = HashMap<&'a str, &'a dyn LuaObject>;

pub struct RuntimeStage {
    lua_objects: LuaObjects<'static>,
    factorio_types: Vec<Box<FactorioType>>,
}

impl RuntimeStage {
    pub fn new() -> Self {
        RuntimeStage {
            lua_objects: Default::default(),
            factorio_types: vec![],
        }
    }

    pub fn add_factorio_type(&mut self, json: &str) -> Result<Option<Box<FactorioType>>> {
        let factorio_type = Box::new(parse_factorio_type(json)?);
        let lua_object_count = self.lua_objects.len();

        // SAFETY: if the lua objects grow, we add factorio_type to the vec, so the reference
        // is always valid, because it references the boxed value within the struct itself.
        add_to_lua_objects(&factorio_type, unsafe {
            mem::transmute(&mut self.lua_objects)
        });
        if self.lua_objects.len() > lua_object_count {
            self.factorio_types.push(factorio_type);
        } else {
            return Ok(Some(factorio_type));
        }
        Ok(None)
    }

    pub fn lua_objects(&self) -> &LuaObjects {
        &self.lua_objects
    }

    pub fn factorio_types(&self) -> &Vec<Box<FactorioType>> {
        &self.factorio_types
    }
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
