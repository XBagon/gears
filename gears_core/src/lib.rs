pub use ty::Type;
pub use value::{Struct, Value, WrapInStruct};

pub mod gear;
mod runtime;
pub mod ty;
pub mod value;

#[cfg(test)]
mod tests {
    #[test]
    fn compiles() {}
}

#[derive(Debug)]
pub enum Error {
    InputTypeMismatch,
    OutputTypeMismatch,
    TriedToDestructureNonStruct(Type),
    Unimplemented,
}

pub type Result<T> = std::result::Result<T, Error>;
