use crate::lua_object::LuaObject;
use std::collections::HashMap;
use std::fmt::Debug;

pub trait Traversable: Debug {
    fn traverse(&self) -> Vec<&dyn Traversable> {
        vec![]
    }

    fn to_lua_object(&self) -> Option<&dyn LuaObject> {
        None
    }

    fn as_traversable(&self) -> &dyn Traversable
    where
        Self: Sized,
    {
        self
    }
}

macro_rules! traversable {
    ($t:ident) => { impl Traversable for $t {} };
    ($($t:ident),+) => { $(traversable!($t);)+ };
}

traversable!(i8, i16, i32);
traversable!(u8, u16, u32, u64);
traversable!(f32, f64, bool, String);

impl<T: Traversable> Traversable for Box<T> {
    fn traverse(&self) -> Vec<&dyn Traversable> {
        vec![self.as_ref()]
    }
}

impl<T: Traversable> Traversable for Option<T> {
    fn traverse(&self) -> Vec<&dyn Traversable> {
        match self {
            None => vec![],
            Some(value) => vec![value],
        }
    }
}

impl<T: Traversable> Traversable for Vec<T> {
    fn traverse(&self) -> Vec<&dyn Traversable> {
        self.iter().map(|v| v as &dyn Traversable).collect()
    }
}

impl<K: Debug, V: Traversable> Traversable for HashMap<K, V> {
    fn traverse(&self) -> Vec<&dyn Traversable> {
        self.values().map(|v| v as &dyn Traversable).collect()
    }
}

impl<A: Traversable, B: Traversable> Traversable for (A, B) {
    fn traverse(&self) -> Vec<&dyn Traversable> {
        vec![&self.0, &self.1]
    }
}

impl<A: Traversable, B: Traversable, C: Traversable> Traversable for (A, B, C) {
    fn traverse(&self) -> Vec<&dyn Traversable> {
        vec![&self.0, &self.1, &self.2]
    }
}

impl<A: Traversable, B: Traversable, C: Traversable, D: Traversable> Traversable for (A, B, C, D) {
    fn traverse(&self) -> Vec<&dyn Traversable> {
        vec![&self.0, &self.1, &self.2, &self.3]
    }
}
