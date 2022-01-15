use super::*;

pub mod io;
pub mod literal;

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

#[enum_dispatch]
pub enum GearSpecial {
    Input,
    Output,
    Literal,
}
