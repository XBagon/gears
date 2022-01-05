use super::*;
use crate::gear::Error::NonEvaluable;

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
    Input,
    Output,
}

impl Geared for GearSpecial {
    fn evaluate(
        &self,
        _register: &GearRegister,
        input: Vec<TypedValue>,
    ) -> Result<Vec<TypedValue>> {
        match self {
            GearSpecial::Output => Ok(input),
            _ => Err(NonEvaluable),
        }
    }
}
