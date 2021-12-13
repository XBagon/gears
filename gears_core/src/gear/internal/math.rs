use crate::gear::*;
use super::*;

template!(Add (summand1: U64, summand2: U64) -> (sum: U64) {{
    sum = summand1 + summand2
}});

template!(Sub (minuend: U64, subtrahend: U64) -> (difference: U64) {{
    difference = minuend - subtrahend
}});

template!(Mul (factor1: U64, factor2: U64) -> (product: U64) {{
    product = factor1 * factor2
}});

template!(Div (dividend: U64, divisor: U64) -> (fraction: U64) {{
    fraction = dividend / divisor
}});

pub fn init(register: &mut GearRegister) {
    register.0.insert(Add::template());
    register.0.insert(Sub::template());
    register.0.insert(Mul::template());
    register.0.insert(Div::template());
}