pub mod gear;
pub mod ty;
//FIXME: endless loop when connecting wrong index => add checks + typechecks

#[cfg(test)]
mod tests {
    use crate::gear::command::GearCommand;
    use crate::gear::compound::GearCompound;
    use crate::gear::special::literal::Literal;
    use crate::gear::*;
    use crate::ty::*;

    fn squared_gear(register: &mut GearRegister) -> GearId {
        let mut compound = GearCompound::new(register, 1, 1);
        let mul = register.instantiate(register.internal.math_gears.mul);

        compound.connect(compound.input_id, 0, mul, 0);
        compound.connect(compound.input_id, 0, mul, 1);
        compound.connect(mul, 0, compound.output_id, 0);

        register
            .builder(compound.into())
            .name(String::from("Squared"))
            .input(IOInformation::new(
                String::from("base"),
                TypedValue::U64(Default::default()).ty(),
            ))
            .output(IOInformation::new(
                String::from("square"),
                TypedValue::U64(Default::default()).ty(),
            ))
            .instantiate()
    }

    #[test]
    fn test_squared_gear() {
        let mut register = GearRegister::init();

        let gear = squared_gear(&mut register);

        assert_eq!(
            register.evaluate(gear, vec![TypedValue::U64(5)]).unwrap()[0],
            TypedValue::U64(25)
        );
    }

    #[test]
    fn test_x2_plus_y2() {
        let mut register = GearRegister::init();
        let squared = squared_gear(&mut register);

        let mut compound = GearCompound::new(&mut register, 1, 1);
        let squared_x = register.instantiate(squared);
        let squared_y = register.instantiate(squared);
        let add = register.instantiate(register.internal.math_gears.add);

        compound.connect(compound.input_id, 0, squared_x, 0);
        compound.connect(compound.input_id, 1, squared_y, 0);
        compound.connect(squared_x, 0, add, 0);
        compound.connect(squared_y, 0, add, 1);
        compound.connect(add, 0, compound.output_id, 0);

        let gear = register
            .builder(compound.into())
            .name(String::from("x² + y² = z²"))
            .input(IOInformation::new(
                String::from("x"),
                TypedValue::U64(Default::default()).ty(),
            ))
            .input(IOInformation::new(
                String::from("y"),
                TypedValue::U64(Default::default()).ty(),
            ))
            .output(IOInformation::new(
                String::from("z²"),
                TypedValue::U64(Default::default()).ty(),
            ))
            .instantiate();
        assert_eq!(
            register
                .evaluate(gear, vec![TypedValue::U64(4), TypedValue::U64(6)])
                .unwrap()[0],
            TypedValue::U64(52)
        );
    }

    #[test]
    fn test_echo() {
        let mut register = GearRegister::init();
        let command = GearCommand::new(String::from("echo"));

        let gear = register
            .builder(command.into())
            .name(String::from("Echo"))
            .input(IOInformation::new(
                String::from("text"),
                TypedValue::String(Default::default()).ty(),
            ))
            .output(IOInformation::new(
                String::from("exit code"),
                TypedValue::String(Default::default()).ty(),
            ))
            .output(IOInformation::new(
                String::from("stdout"),
                TypedValue::String(Default::default()).ty(),
            ))
            .output(IOInformation::new(
                String::from("stderr"),
                TypedValue::String(Default::default()).ty(),
            ))
            .instantiate();

        assert_eq!(
            register
                .evaluate(gear, vec![TypedValue::String(String::from("Hello world!"))])
                .unwrap()[1],
            TypedValue::String(String::from("Hello world!\n"))
        );
    }

    #[test]
    fn test_cargo() {
        let mut register = GearRegister::init();
        let gear = register.instantiate(register.command.generic_command);

        assert_eq!(
            register
                .evaluate(gear, vec![TypedValue::String(String::from("cargo"))])
                .unwrap()[0],
            TypedValue::I32(0)
        );
    }

    #[test]
    fn test_increment() {
        let mut register = GearRegister::init();

        let mut compound = GearCompound::new(&mut register, 1, 1);
        let add = register
            .instantiator(register.internal.math_gears.add)
            .instantiate();

        let one = Literal::instantiate(&mut register, TypedValue::U64(1));

        compound.connect(compound.input_id, 0, add, 0);
        compound.connect(one, 0, add, 1);
        compound.connect(add, 0, compound.output_id, 0);

        let gear = register
            .builder(compound.into())
            .name(String::from("Increment"))
            .input(IOInformation::new(
                String::from("number"),
                TypedValue::I32(Default::default()).ty(),
            ))
            .output(IOInformation::new(
                String::from("incremented"),
                TypedValue::I32(Default::default()).ty(),
            ))
            .instantiate();

        assert_eq!(
            register.evaluate(gear, vec![TypedValue::U64(0)]).unwrap()[0],
            TypedValue::U64(1)
        );
    }
}
