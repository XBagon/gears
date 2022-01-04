pub mod gear;

//TODO: `.get(_).unwrap()` -> `[_]` ?
//FIXME: endless loop when connecting wrong index => add checks + typechecks

#[cfg(test)]
mod tests {
    use crate::gear::*;
    use crate::gear::compound::GearCompound;

    /*
    #[test]
    fn it_works() {
        let register = GearRegister::init(); //TODO: how to get right gear?
        let add = register.gears.iter().next().unwrap().1;
        let input = vec![TypedValue::U64(2), TypedValue::U64(2)];
        let sum = if let &TypedValue::U64(sum) = add.evaluate(input).unwrap().first().unwrap() {
            sum
        } else { panic!() };
        assert_eq!(sum, 4);
    }
    */

    #[test]
    fn it_compiles() {}

    fn squared_gear(register: &GearRegister) -> Gear {
        let mut compound = GearCompound::new(&register, 1, 1);
        let mul = compound.add_gear(register.internal.math_gears.mul.instance());

        compound.connect(compound.input_id, 0, mul, 0);
        compound.connect(compound.input_id, 0, mul, 1);
        compound.connect(mul, 0, compound.output_id, 0);

        Gear::new(
            String::from("Squared"),
            vec![IOInformation::new(String::from("base"), TypedValue::U64(Default::default()).ty())],
            vec![IOInformation::new(String::from("square"), TypedValue::U64(Default::default()).ty())],
            compound.into()
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
            vec![IOInformation::new(String::from("x"), TypedValue::U64(Default::default()).ty()), IOInformation::new(String::from("y"), TypedValue::U64(Default::default()).ty())],
            vec![IOInformation::new(String::from("z²"), TypedValue::U64(Default::default()).ty())],
            compound.into()
        );
        assert_eq!(
            gear.evaluate(&register, vec![TypedValue::U64(4), TypedValue::U64(6)]).unwrap()[0],
            TypedValue::U64(52)
        );
    }
}