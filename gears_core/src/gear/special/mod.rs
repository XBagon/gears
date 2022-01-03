use io::*;
use crate::gear::Error::NonEvaluable;
use super::*;

pub mod io;

pub struct Gears {
    pub io: io::Gears,
}

impl Gears {
    pub fn init(gears: &mut GearSlotMap) -> Self {
        Self {
            io: io::Gears::init(gears),
        }
    }
}

pub enum GearSpecial {
    Input(Input),
    Output(Output),
}

impl Geared for GearSpecial {
    fn evaluate(&self, _register: &GearRegister, _input: Vec<TypedValue>) -> Result<Vec<TypedValue>> {
        Err(NonEvaluable)
    }
}