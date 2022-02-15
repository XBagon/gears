use super::*;

#[derive(Clone)]
pub struct Literal(pub TypedValue);
impl Literal {
    pub fn instantiate(register: &mut GearRegister, value: TypedValue) -> GearId {
        register.register(Gear {
            name: String::from("Literal"),
            inputs: Vec::new(),
            outputs: vec![IOInformation::new(String::from("value"), value.ty())],
            implementation: GearImplementation::GearSpecial(GearSpecial::Literal(Literal(value))),
        })
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
