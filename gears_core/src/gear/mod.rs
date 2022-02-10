use crate::gear::special::GearSpecial;
use crate::gear::{
    command::{GearCommand, GearGenericCommand},
    compound::GearCompound,
    internal::GearInternal,
    special::{io::Input, io::Output, literal::Literal},
};
use crate::ty::*;
use enum_dispatch::enum_dispatch;
use slotmap::new_key_type;
use thiserror::Error;
use crate::util::LiftSlotMap;

pub mod command;
pub mod compound;
pub mod internal;
pub mod special;

new_key_type! { pub struct GearId; }

impl Geared for GearId {
    fn evaluate(&self, register: &GearRegister, input: Vec<TypedValue>) -> Result<Vec<TypedValue>> {
        register.evaluate(*self, input)
    }
}

type GearSlotMap = LiftSlotMap<GearId, Gear>;

pub struct GearRegister {
    pub gears: GearSlotMap,
    pub internal: internal::Gears,
    pub special: special::Gears,
    pub command: command::Gears,
}

impl GearRegister {
    pub fn init() -> Self {
        let mut gears = LiftSlotMap::with_key().into();
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

    pub fn instantiate(&mut self, template_gear_id: GearId) -> GearId {
        self.register(self.templated_gear(template_gear_id))
    }

    pub fn instantiator(&mut self, template_gear_id: GearId) -> GearBuilder {
        GearBuilder {
            gear: self.templated_gear(template_gear_id),
            register: self,
        }
    }

    pub fn builder(&mut self, implementation: GearImplementation) -> GearBuilder {
        let gear = Gear {
            name: String::new(),
            inputs: vec![],
            outputs: vec![],
            implementation,
        };
        GearBuilder {
            gear,
            register: self,
        }
    }

    fn templated_gear(&self, template_gear_id: GearId) -> Gear {
        let template = &self.gears[template_gear_id];
        Gear {
            name: template.name.clone(),
            inputs: template.inputs.clone(),
            outputs: template.outputs.clone(),
            implementation: GearImplementation::Template(template_gear_id),
        }
    }

    pub fn get_mut_template_implementation(&mut self, gear_id: GearId) -> Option<&mut GearImplementation> {
        self.get_template_gear_id(gear_id).map(move |id| {
            &mut self.gears[id].implementation
        })
    }

    pub fn get_template_gear_id(&self, gear_id: GearId) -> Option<GearId> {
        if let GearImplementation::Template(template_gear_id) = self.gears[gear_id].implementation {
            Some(template_gear_id)
        } else {
            None
        }
    }

    pub fn lift_gear(&mut self) -> () {

    }

    pub fn evaluate(&self, gear_id: GearId, input: Vec<TypedValue>) -> Result<Vec<TypedValue>> {
        self.gears[gear_id].evaluate(self, input)
    }
}

pub struct GearBuilder<'a> {
    register: &'a mut GearRegister,
    pub gear: Gear,
}
impl<'a> GearBuilder<'a> {
    pub fn instantiate(self) -> GearId {
        self.register.register(self.gear)
    }

    pub fn name(mut self, name: String) -> Self {
        self.gear.name = name;
        self
    }

    pub fn input(mut self, io_info: IOInformation) -> Self {
        self.gear.inputs.push(io_info);
        self
    }

    pub fn output(mut self, io_info: IOInformation) -> Self {
        self.gear.outputs.push(io_info);
        self
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

impl Geared for Gear {
    fn evaluate(&self, register: &GearRegister, input: Vec<TypedValue>) -> Result<Vec<TypedValue>> {
        self.implementation.evaluate(register, input)
    }
}

#[derive(Clone)]
pub struct IOInformation {
    pub name: String,
    pub ty: TypeDiscriminant,
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
    Template(GearId),
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

#[enum_dispatch(GearImplementation, GearSpecial)]
pub trait Geared {
    fn evaluate(&self, register: &GearRegister, input: Vec<TypedValue>) -> Result<Vec<TypedValue>>;
}
