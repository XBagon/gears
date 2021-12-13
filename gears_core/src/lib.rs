pub mod gear;

#[cfg(test)]
mod tests {
    use crate::gear::*;

    #[test]
    fn it_works() {
        let mut register = GearRegister::new();
        register.init();
        let add = register.0.iter().next().unwrap().1;
        let input = vec![TypedValue::U64(2), TypedValue::U64(2)];
        let sum = if let &TypedValue::U64(sum) = add.evaluate(input).first().unwrap() {
            sum
        } else { panic!() };
        assert_eq!(sum, 4);
    }
}