use core::default::Default;

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

pub type TypeDiscriminant = std::mem::Discriminant<TypedValue>;

//TODO: use lazy_static to create constant `TypeDiscriminant`s for each type, until `std::mem::discriminant` is const on stable
impl TypedValue {
    pub fn ty(&self) -> TypeDiscriminant {
        std::mem::discriminant(self)
    }
}
