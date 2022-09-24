use crate::ty::StructType;
use crate::{Error, Type};
use derive_more::{From, TryInto};
use std::convert::TryInto;
use std::ops::Index;

#[derive(Clone, Debug, PartialEq, From, TryInto)]
#[try_into(ref)]
pub enum Value {
    Float(f32),
    Struct(Struct),
    #[allow(dead_code)]
    Unimplemented,
}

impl Value {
    pub fn from_vec(vec: Vec<Value>) -> Self {
        Self::Struct(vec.into())
    }

    pub fn ty(&self) -> Type {
        match self {
            Value::Float(_) => Type::Float,
            Value::Struct(strct) => strct.ty(),
            _ => unimplemented!(),
        }
    }

    pub fn to_struct(&self) -> crate::Result<&Struct> {
        self.try_into()
            .map_err(|_| Error::TriedToDestructureNonStruct(self.ty()))
    }

    pub fn into_struct(self) -> crate::Result<Struct> {
        let ty = self.ty();
        self.try_into()
            .map_err(|_| Error::TriedToDestructureNonStruct(ty))
    }
}

impl From<Vec<Value>> for Value {
    fn from(vec: Vec<Value>) -> Self {
        Self::from_vec(vec)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Struct(pub Vec<Value>);

impl Index<usize> for Struct {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Struct {
    pub(crate) fn ty(&self) -> Type {
        Type::Struct(StructType(self.0.iter().map(|f| f.ty()).collect()))
    }
}

impl From<Vec<Value>> for Struct {
    fn from(vec: Vec<Value>) -> Self {
        Self(vec)
    }
}

pub trait WrapInStruct {
    fn wrap_in_struct(self) -> Struct;
}

impl WrapInStruct for Vec<Value> {
    fn wrap_in_struct(self) -> Struct {
        self.into()
    }
}

impl WrapInStruct for Value {
    fn wrap_in_struct(self) -> Struct {
        vec![self].into()
    }
}
