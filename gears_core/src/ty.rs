#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Float,
    Struct(StructType),
    #[allow(dead_code)]
    Unimplemented,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructType(pub Vec<Type>);
