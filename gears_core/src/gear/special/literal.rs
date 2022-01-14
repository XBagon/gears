use super::*;

pub struct Gears {
    pub literal: GearId,
}

impl Gears {
    pub fn init(gears: &mut GearSlotMap) -> Self {
        Self {
            literal: gears.insert(Literal::template()),
        }
    }
}

pub struct Literal(TypedValue);
impl Literal {
    pub fn template() -> Gear {
        Gear {
            name: String::from("Literal"),
            inputs: Vec::new(),
            outputs: Vec::new(),
            implementation: GearImplementation::GearSpecial(GearSpecial::Literal(Literal(
                TypedValue::None,
            ))),
        }
    }
}

impl Geared for Literal {
    fn evaluate(
        &self,
        _register: &GearRegister,
        _input: Vec<TypedValue>,
    ) -> Result<Vec<TypedValue>> {
        Ok(vec![self.0.clone()])
    }
}
