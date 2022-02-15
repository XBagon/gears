use super::*;

pub mod io;
pub mod literal;

pub struct Gears {
    pub io: io::Gears,
}

impl Gears {
    pub fn init(template_gears: &mut TemplateGearMap) -> Self {
        Self {
            io: io::Gears::init(template_gears),
        }
    }
}

#[enum_dispatch]
#[derive(Clone)]
pub enum GearSpecial {
    Input,
    Output,
    Literal,
}
