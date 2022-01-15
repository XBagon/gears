use super::*;

pub struct Gears {
    pub add: GearId,
    pub sub: GearId,
    pub mul: GearId,
    pub div: GearId,
}

impl Gears {
    pub fn init(gears: &mut GearSlotMap) -> Self {
        Self {
            add: gears.insert(Add::template()),
            sub: gears.insert(Sub::template()),
            mul: gears.insert(Mul::template()),
            div: gears.insert(Div::template()),
        }
    }
}

//TODO: define for other num types

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
