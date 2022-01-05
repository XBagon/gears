use crate::gear::special::GearSpecial;
use crate::{
    gear::command::{GearCommand, GearGenericCommand},
    gear::compound::GearCompound,
    gear::internal::GearInternal,
};
use enum_dispatch::enum_dispatch;
use slotmap::{new_key_type, SlotMap};
use thiserror::Error;

pub mod command;
pub mod compound;
pub mod internal;
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
    pub command: command::Gears,
}

impl GearRegister {
    pub fn init() -> Self {
        let mut gears = SlotMap::with_key();
        Self {
            internal: internal::Gears::init(&mut gears),
            special: special::Gears::init(&mut gears),
            command: command::Gears::init(&mut gears),
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
    pub fn new(
        name: String,
        inputs: Vec<IOInformation>,
        outputs: Vec<IOInformation>,
        implementation: GearImplementation,
    ) -> Self {
        Gear {
            name,
            inputs,
            outputs,
            implementation,
        }
    }
}

impl Geared for Gear {
    fn evaluate(&self, register: &GearRegister, input: Vec<TypedValue>) -> Result<Vec<TypedValue>> {
        self.implementation.evaluate(register, input)
    }
}

#[derive(Debug)]
pub struct GearInstance {
    /// Overwrites [gear][`Self::gear`]'s name
    pub name: Option<String>,
    /// Overwrites [gear][`Self::gear`]'s input names
    pub input_names: Vec<Option<String>>,
    /// Overwrites [gear][`Self::gear`]'s output names
    pub output_names: Vec<Option<String>>,
    /// Id of the template [`Gear`]
    pub gear: GearId,
}

impl GearInstance {
    pub fn new(gear: GearId) -> Self {
        Self {
            name: None,
            input_names: vec![],
            output_names: vec![],
            gear,
        }
    }
}

impl Geared for GearInstance {
    fn evaluate(&self, register: &GearRegister, input: Vec<TypedValue>) -> Result<Vec<TypedValue>> {
        register.gears[self.gear].evaluate(register, input)
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
    GearCommand,
    GearGenericCommand,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error occurred in evaluation")]
    GearInternalError(#[from] Box<dyn std::error::Error>),
    #[error("IOError occured")]
    IOError(#[from] std::io::Error),
    #[error("IOError occured")]
    FromUTF8Error(#[from] std::string::FromUtf8Error),
    #[error("This `GearSpecial` isn't evaluable")]
    NonEvaluable,
    #[error("Terminated by signal: {0}")]
    TerminatedBySignal(i32),
}

pub type Result<T> = std::result::Result<T, Error>;

#[enum_dispatch(GearImplementation)]
pub trait Geared {
    fn evaluate(&self, register: &GearRegister, input: Vec<TypedValue>) -> Result<Vec<TypedValue>>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypedValue {
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
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
