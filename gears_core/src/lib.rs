pub mod gear;

//TODO: `.get(_).unwrap()` -> `[_]` ?
//FIXME: endless loop when connecting wrong index => add checks + typechecks

#[cfg(test)]
mod tests {
    use crate::gear::command::GearCommand;
    use crate::gear::compound::GearCompound;
    use crate::gear::*;

    fn squared_gear(register: &GearRegister) -> Gear {
        let mut compound = GearCompound::new(&register, 1, 1);
        let mul = compound.add_gear(register.internal.math_gears.mul.instance());

        compound.connect(compound.input_id, 0, mul, 0);
        compound.connect(compound.input_id, 0, mul, 1);
        compound.connect(mul, 0, compound.output_id, 0);

        Gear::new(
            String::from("Squared"),
            vec![IOInformation::new(
                String::from("base"),
                TypedValue::U64(Default::default()).ty(),
            )],
            vec![IOInformation::new(
                String::from("square"),
                TypedValue::U64(Default::default()).ty(),
            )],
            compound.into(),
        )
    }

    #[test]
    fn test_squared_gear() {
        let register = GearRegister::init();

        let gear = squared_gear(&register);

        assert_eq!(
            gear.evaluate(&register, vec![TypedValue::U64(5)]).unwrap()[0],
            TypedValue::U64(25)
        );
    }

    #[test]
    fn test_x2_plus_y2() {
        let mut register = GearRegister::init();
        let squared_gear = squared_gear(&register);
        let squared_id = register.register(squared_gear);

        let mut compound = GearCompound::new(&register, 1, 1);
        let squared_x = compound.add_gear(squared_id.instance());
        let squared_y = compound.add_gear(squared_id.instance());
        let add = compound.add_gear(register.internal.math_gears.add.instance());

        compound.connect(compound.input_id, 0, squared_x, 0);
        compound.connect(compound.input_id, 1, squared_y, 0);
        compound.connect(squared_x, 0, add, 0);
        compound.connect(squared_y, 0, add, 1);
        compound.connect(add, 0, compound.output_id, 0);

        let gear = Gear::new(
            String::from("x² + y²"),
            vec![
                IOInformation::new(String::from("x"), TypedValue::U64(Default::default()).ty()),
                IOInformation::new(String::from("y"), TypedValue::U64(Default::default()).ty()),
            ],
            vec![IOInformation::new(
                String::from("z²"),
                TypedValue::U64(Default::default()).ty(),
            )],
            compound.into(),
        );
        assert_eq!(
            gear.evaluate(&register, vec![TypedValue::U64(4), TypedValue::U64(6)])
                .unwrap()[0],
            TypedValue::U64(52)
        );
    }

    #[test]
    fn test_echo() {
        let register = GearRegister::init();
        let command = GearCommand::new(String::from("echo"));

        let gear = Gear::new(
            String::from("Echo"),
            vec![IOInformation::new(
                String::from("text"),
                TypedValue::String(Default::default()).ty(),
            )],
            vec![
                IOInformation::new(
                    String::from("exit code"),
                    TypedValue::String(Default::default()).ty(),
                ),
                IOInformation::new(
                    String::from("stdout"),
                    TypedValue::String(Default::default()).ty(),
                ),
                IOInformation::new(
                    String::from("stderr"),
                    TypedValue::String(Default::default()).ty(),
                ),
            ],
            command.into(),
        );

        assert_eq!(
            gear.evaluate(
                &register,
                vec![TypedValue::String(String::from("Hello world!"))]
            )
            .unwrap()[1],
            TypedValue::String(String::from("Hello world!\n"))
        );
    }

    #[test]
    fn test_cargo() {
        let register = GearRegister::init();
        let gear = register.command.generic_command.instance();

        assert_eq!(
            gear.evaluate(&register, vec![TypedValue::String(String::from("cargo"))])
                .unwrap()[0],
            TypedValue::I32(0)
        );
    }
}
