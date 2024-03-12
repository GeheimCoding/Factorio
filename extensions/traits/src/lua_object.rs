use std::fmt::Debug;

pub trait LuaObjectClassId {
    fn class_id(&self) -> &str;
}

pub trait LuaObject: Debug + LuaObjectClassId {}

impl<T: Debug + LuaObjectClassId> LuaObject for T {}
