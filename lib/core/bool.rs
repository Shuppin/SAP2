use crate::core::{Int, Str, Float};

pub struct Bool {
    pub value: bool,
}
impl_into!(Bool => Int, |self| { Ok(Int { value: self.value as i32 }) });
impl_into!(Bool => Float, |self| { Ok(Float { value: self.value as i32 as f32 }) });
impl_into!(Bool => Str, |self| { Ok(Str { value: self.value.to_string() }) });