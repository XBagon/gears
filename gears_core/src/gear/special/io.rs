use super::*;
use crate::gear::Error::NonEvaluable;

pub struct Gears {
    pub input: TemplateGearId,
    pub output: TemplateGearId,
}

impl Gears {
    pub fn init(template_gears: &mut TemplateGearMap) -> Self {
        Self {
            input: template_gears.insert(Input::template()),
            output: template_gears.insert(Output::template()),
        }
    }
}

#[derive(Clone)]
pub struct Input;
impl Input {
    pub fn template() -> Gear {
        Gear {
            name: String::from("Input"),
            inputs: Vec::new(),
            outputs: Vec::new(),
            implementation: GearImplementation::GearSpecial(GearSpecial::Input(Input)),
        }
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

#[derive(Clone)]
pub struct Output;
impl Output {
    pub fn template() -> Gear {
        Gear {
            name: String::from("Output"),
            inputs: Vec::new(),
            outputs: Vec::new(),
            implementation: GearImplementation::GearSpecial(GearSpecial::Output(Output)),
        }
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
