macro_rules! impl_into {
    ($from:ty => $to:ty, |$var:ident| $code:block) => {
        impl TryInto<$to> for $from {
            type Error = ();
            fn try_into($var: $from) -> Result<$to, Self::Error> {
                $code
            }
        }
    };
}

pub mod object;
pub use object::Object;

pub mod bool;
pub mod float;
pub mod list;
pub mod str;
pub mod int;

pub use self::bool::Bool;
pub use float::Float;
pub use list::List;
pub use self::str::Str;
pub use int::Int;