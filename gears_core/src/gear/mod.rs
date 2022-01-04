use slotmap::{SlotMap, new_key_type};
use enum_dispatch::enum_dispatch;
use crate::{
    gear::compound::GearCompound,
    gear::internal::GearInternal
};
use crate::gear::special::GearSpecial;
use thiserror::Error;

pub mod internal;
pub mod compound;
pub mod special;

new_key_type! { pub struct GearId; }

impl GearId {
    pub fn instance(self) -> GearInstance {
        GearInstance::new(self)
    }
}

type GearSlotMap = SlotMap<GearId, Gear>;

pub struct GearRegister {
    pub gears: GearSlotMap,
    pub internal: internal::Gears,
    pub special: special::Gears,
}

impl GearRegister {
    pub fn init() -> Self {
        let mut gears = SlotMap::with_key();
        Self {
            internal: internal::Gears::init(&mut gears),
            special: special::Gears::init(&mut gears),
            gears,
        }
    }

    pub fn register(&mut self, gear: Gear) -> GearId {
        self.gears.insert(gear)
    }
}

impl Default for GearRegister {
    fn default() -> Self {
        Self::init()
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
    fn evaluate(&self, register: &GearRegister, input: Vec<TypedValue>) -> Result<Vec<TypedValue>> {
        self.implementation.evaluate(register, input)
    }
}

#[derive(Debug)]
pub struct GearInstance {
    pub name: Option<String>,
    pub input_names: Vec<Option<String>>,
    pub output_names: Vec<Option<String>>,
    pub gear: GearId,
}

impl GearInstance {
    pub fn new(gear: GearId) -> Self {
        Self {
            name: None,
            input_names: vec![],
            output_names: vec![],
            gear
        }
    }
}

impl From<GearId> for GearInstance {
    fn from(id: GearId) -> Self {
        id.instance()
    }
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
    GearSpecial,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error occurred in evaluation")]
    GearInternalError(#[from] Box<dyn std::error::Error>),
    #[error("This `GearSpecial` isn't evaluable")]
    NonEvaluable,
}

pub type Result<T> = std::result::Result<T, Error>;

#[enum_dispatch(GearImplementation)]
pub trait Geared {
    fn evaluate(&self, register: &GearRegister, input: Vec<TypedValue>) -> Result<Vec<TypedValue>>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypedValue {
    U64(u64),
    F64(f64),
    String(String),
}

//TODO: use lazy_static to create constant `TypeDiscriminant`s for each type, until `std::mem::discriminant` is const on stable

pub type TypeDiscriminant = std::mem::Discriminant<TypedValue>;

impl TypedValue {
    pub fn ty(&self) -> TypeDiscriminant {
        std::mem::discriminant(self)
    }
}
