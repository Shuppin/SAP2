use crate::core::{Bool, Float, Int, List, Str};

pub enum Object {
    Bool(Bool),
    Float(Float),
    List(List),
    Str(Str),
    Int(Int),
}

macro_rules! impl_into_obj {
    ($ty:ident) => {
        impl Into<Object> for $ty {
            fn into(self) -> Object {
                Object::$ty(self)
            }
        }
    };
}

impl_into_obj!(Bool);
impl_into_obj!(Float);
impl_into_obj!(Int);
impl_into_obj!(List);
impl_into_obj!(Str);
