use crate::core::{Float, Str, Bool};

pub struct Int {
    pub value: i32,
}

impl Int {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}
impl_into!(Int => Float, |self| { Ok(Float { value: self.value as f32 }) });
impl_into!(Int => Str, |self| { Ok(Str { value: self.value.to_string() }) });
impl_into!(Int => Bool, |self| { Ok(Bool { value: self.value != 0 }) });