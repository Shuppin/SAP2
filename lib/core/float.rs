use crate::core::{Int, Str, Bool};

pub struct Float {
    pub value: f32,
}
impl_into!(Float => Int, |self| { Ok(Int { value: self.value as i32 }) });
impl_into!(Float => Str, |self| { Ok(Str { value: self.value.to_string() }) });
impl_into!(Float => Bool, |self| { Ok(Bool { value: self.value != 0.0 }) });