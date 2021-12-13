use slotmap::{SlotMap, new_key_type};
use enum_dispatch::enum_dispatch;
use crate::{
    gear::compound::GearCompound,
    gear::internal::GearInternal
};

mod internal;
mod compound;

new_key_type! { pub struct GearId; }

pub struct GearRegister(pub SlotMap<GearId, Gear>);

impl GearRegister {
    pub fn new() -> Self {
        Self(SlotMap::with_key())
    }

    pub fn init(&mut self) {
        internal::init(self);
    }
}

impl Default for GearRegister {
    fn default() -> Self {
        let mut s = Self::new();
        s.init();
        s
    }
}

pub struct Gear {
    pub name: String,
    pub inputs: Vec<IOInformation>,
    pub outputs: Vec<IOInformation>,
    pub implementation: GearImplementation,
}

impl Gear {
    pub fn new(name: String, inputs: Vec<IOInformation>, outputs: Vec<IOInformation>, implementation: GearImplementation) -> Self {
        Gear { name, inputs, outputs, implementation }
    }
}

impl Geared for Gear {
    fn evaluate(&self, input: Vec<TypedValue>) -> Vec<TypedValue> {
        self.implementation.evaluate(input)
    }
}

pub struct GearInstance {
    pub name: Option<String>,
    pub input_names: Vec<Option<String>>,
    pub output_names: Vec<Option<String>>,
    pub gear: GearId,
}

pub struct IOInformation {
    name: String,
    ty: TypeDiscriminant,
}

impl IOInformation {
    pub fn new(name: String, ty: TypeDiscriminant) -> Self {
        IOInformation { name, ty }
    }
}

#[enum_dispatch]
pub enum GearImplementation {
    GearInternal,
    GearCompound,
}

#[enum_dispatch(GearImplementation)]
pub trait Geared {
    fn evaluate(&self, input: Vec<TypedValue>) -> Vec<TypedValue>;
}

pub enum TypedValue {
    U64(u64),
    F64(f64),
    String(String),
}

pub type TypeDiscriminant = std::mem::Discriminant<TypedValue>;

impl TypedValue {
    fn ty(&self) -> TypeDiscriminant {
        std::mem::discriminant(self)
    }
}
