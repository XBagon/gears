use super::*;
use crate::gear::Error::NonEvaluable;

pub struct Gears {
    pub input: GearId,
    pub output: GearId,
}

impl Gears {
    pub fn init(gears: &mut GearSlotMap) -> Self {
        Self {
            input: gears.insert(Input::template()),
            output: gears.insert(Output::template()),
        }
    }
}

pub struct Input;
impl Input {
    pub fn template() -> Gear {
        Gear::new(
            String::from("Input"),
            Vec::new(),
            Vec::new(),
            GearImplementation::GearSpecial(GearSpecial::Input(Input)),
        )
    }
}

impl Geared for Input {
    fn evaluate(
        &self,
        _register: &GearRegister,
        _input: Vec<TypedValue>,
    ) -> Result<Vec<TypedValue>> {
        Err(NonEvaluable)
    }
}

pub struct Output;
impl Output {
    pub fn template() -> Gear {
        Gear::new(
            String::from("Output"),
            Vec::new(),
            Vec::new(),
            GearImplementation::GearSpecial(GearSpecial::Output(Output)),
        )
    }
}

impl Geared for Output {
    fn evaluate(
        &self,
        _register: &GearRegister,
        input: Vec<TypedValue>,
    ) -> Result<Vec<TypedValue>> {
        Ok(input)
    }
}
