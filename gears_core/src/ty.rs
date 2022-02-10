use core::default::Default;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum TypedValue {
    None,
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    F64(f64),
    String(String),
}

impl Default for TypedValue {
    fn default() -> Self {
        TypedValue::None
    }
}

impl Display for TypedValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypedValue::None => write!(f, ""),
            TypedValue::U32(n) => n.fmt(f),
            TypedValue::U64(n) => n.fmt(f),
            TypedValue::I32(n) => n.fmt(f),
            TypedValue::I64(n) => n.fmt(f),
            TypedValue::F64(n) => n.fmt(f),
            TypedValue::String(s) => s.fmt(f),
        }
    }
}

pub type TypeDiscriminant = std::mem::Discriminant<TypedValue>;

//TODO: use lazy_static to create constant `TypeDiscriminant`s for each type, until `std::mem::discriminant` is const on stable
impl TypedValue {
    pub fn ty(&self) -> TypeDiscriminant {
        std::mem::discriminant(self)
    }
}
