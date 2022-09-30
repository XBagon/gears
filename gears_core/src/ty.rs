use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Float,
    Struct(StructType),
    #[allow(dead_code)]
    Unimplemented,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct StructType(pub Vec<Type>);
