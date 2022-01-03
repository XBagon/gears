use slotmap::{SlotMap, new_key_type};
use enum_dispatch::enum_dispatch;
use crate::{
    gear::compound::GearCompound,
    gear::internal::GearInternal
};

mod internal;
mod compound;
mod special;

new_key_type! { pub struct GearId; }

type GearSlotMap = SlotMap<GearId, Gear>;

pub struct GearRegister {
    pub gears: GearSlotMap,
    internal: internal::Gears,
    special: special::Gears,
}

impl GearRegister {
    pub fn init() -> Self {
        let mut gears = SlotMap::with_key();
        Self {
            gears,
            internal: internal::Gears::init(&mut gears),
            special: special::Gears::init(&mut gears),
        }
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
        GearInstance::new(id)
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
