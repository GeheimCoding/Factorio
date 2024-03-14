use std::any::Any;
use std::fmt::Debug;

pub trait LuaObjectClassId {
    fn class_id(&self) -> &str;
}

pub trait LuaObject: Any + Debug + LuaObjectClassId {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any + Debug + LuaObjectClassId> LuaObject for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
