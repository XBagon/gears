use core::default::Default;
use std::fmt::{Display, Formatter};

macro_rules! types {
    ($($name:ident($value:ty),)*) => {
        #[derive(Clone, Debug)]
        pub enum TypedValue {
            None,
            $($name($value),)*
        }
        impl Default for TypedValue {
            fn default() -> Self {
                TypedValue::None
            }
        }
        impl TypedValue {
            pub fn ty(&self) -> Type {
                match self {
                    TypedValue::None => Type::None,
                    $(TypedValue::$name(_) => Type::$name,)*
                }
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub enum Type {
            None,
            $($name,)*
        }
        impl Default for Type {
            fn default() -> Self {
                Type::None
            }
        }
        impl Type {
            pub fn val(&self) -> TypedValue {
                match self {
                    Type::None => TypedValue::None,
                    $(Type::$name => TypedValue::$name(<$value>::default()),)*
                }
            }
        }
    }
}

types!{
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    F64(f64),
    String(String),
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
