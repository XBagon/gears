pub mod gear;

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

    #[test]
    fn gear_compound() {
        let mut register = GearRegister::init();
        let mut compound = GearCompound::new(&register, 1, 1);
        let mul = compound.add_gear(register.internal.math_gears.mul.into());
        compound.connect(compound.input_id, 0, mul, 0);
        compound.connect(compound.input_id, 0, mul, 1);
        compound.connect(mul, 0, compound.output_id, 0);
        let gear = Gear::new(
            String::from("Squared"),
            vec![IOInformation::new(String::from("base"), TypedValue::U64(Default::default()).ty())],
            vec![IOInformation::new(String::from("square"), TypedValue::U64(Default::default()).ty())],
            compound.into()
        );
        register.register(gear);
        //TODO: evaluate
    }
}