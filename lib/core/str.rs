use crate::core::{Int, Float, Bool};

pub struct Str {
    pub value: String,
}
impl_into!(Str => Int, |self| {
    self.value
        .parse::<i32>()
        .map(|value| Int { value })
        .map_err(|_| ())
});
impl_into!(Str => Float, |self| {
    self.value
        .parse::<f32>()
        .map(|value| Float { value })
        .map_err(|_| ())
});
impl_into!(Str => Bool, |self| {
    self.value
        .parse::<bool>()
        .map(|value| Bool { value })
        .map_err(|_| ())
});